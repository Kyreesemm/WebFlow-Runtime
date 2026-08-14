use crate::config::{AppConfig, Config};
use crate::window::{build_webview, WindowFactory, WindowFrameStyle, WindowOptions};
use tao::event_loop::{ControlFlow, EventLoop};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wry::{WebContext, WebViewBuilder};

const LINUX_CHROME_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const WINDOWS_CHROME_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const IPHONE_SAFARI_UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Mobile/15E148 Safari/604.1";
const ANDROID_CHROME_UA: &str = "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.200 Mobile Safari/537.36";
const CHROME_LINUX_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36";
const CHROME_WINDOWS_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36";

pub fn run_app(app_id: String, debug: bool) -> Result<(), String> {
    let config: AppConfig = Config::load_app_config(&app_id)
        .ok_or_else(|| format!("App configuration not found for '{}'", app_id))?;

    let event_loop = EventLoop::new();
    let frame_style = if config.window.custom_frame {
        WindowFrameStyle::Custom
    } else {
        WindowFrameStyle::System
    };
    let factory = WindowFactory::new(frame_style);
    let options = WindowOptions {
        title: config.window.title.clone(),
        width: config.window.width,
        height: config.window.height,
        resizable: config.window.resizable,
        frame_style,
        debug,
        position: None,
    };
    let window = factory.create_window(&event_loop, &options)?;

    let storage_dir = if config.isolated_storage {
        Config::get_apps_dir().join(&app_id).join("storage")
    } else {
        Config::get_shared_storage_dir()
    };
    std::fs::create_dir_all(&storage_dir).map_err(|e| e.to_string())?;

    let mut web_context = WebContext::new(Some(storage_dir));
    let mut builder = WebViewBuilder::new_with_web_context(&mut web_context);

    // Apply the same built-in UA profiles exposed by the manager UI.
    let selected_ua = match config.user_agent.as_str() {
        "linux" => Some(LINUX_CHROME_UA),
        "windows" => Some(WINDOWS_CHROME_UA),
        "iphone" => Some(IPHONE_SAFARI_UA),
        "android" => Some(ANDROID_CHROME_UA),
        "chrome-linux" => Some(CHROME_LINUX_UA),
        "chrome-windows" => Some(CHROME_WINDOWS_UA),
        "custom" => config.custom_user_agent.as_deref().filter(|ua| !ua.is_empty()),
        _ => None,
    };
    if let Some(ua) = selected_ua {
        builder = builder.with_user_agent(ua);
    }

    // A small renderer heartbeat lets the native process detect a crashed or
    // wedged WebKit web process and clear its PID marker instead of leaving a
    // permanently "running" app in the manager.
    let heartbeat = Arc::new(AtomicU64::new(now_seconds()));
    let heartbeat_for_ipc = heartbeat.clone();
    builder = builder.with_ipc_handler(move |request| {
        if request.body() == "__webflow_runtime_heartbeat__" {
            heartbeat_for_ipc.store(now_seconds(), Ordering::Relaxed);
        }
    });

    // Custom CSS & JS via initialization scripts.
    let mut init_script = String::new();
    init_script.push_str(
        r#"
        setInterval(function() {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage('__webflow_runtime_heartbeat__');
            }
        }, 2000);
        "#,
    );
    if config.custom_scrollbar {
        init_script.push_str(
            r#"
            (function() {
                const style = document.createElement('style');
                style.textContent = `
                    ::-webkit-scrollbar { width: 8px; height: 8px; }
                    ::-webkit-scrollbar-track { background: #1e1e1e; }
                    ::-webkit-scrollbar-thumb { background: #424242; border-radius: 4px; }
                    ::-webkit-scrollbar-thumb:hover { background: #616161; }
                `;
                document.documentElement.appendChild(style);
            })();
            "#,
        );
    }
    if let Some(ref css) = config.custom_css {
        if !css.trim().is_empty() {
            init_script.push_str(&format!(
                r#"
                (function() {{
                    const style = document.createElement('style');
                    style.textContent = {};
                    document.documentElement.appendChild(style);
                }})();
                "#,
                serde_json::to_string(css).unwrap_or_default()
            ));
        }
    }
    if let Some(ref js) = config.custom_js {
        if !js.trim().is_empty() {
            init_script.push_str(js);
        }
    }
    if !init_script.is_empty() {
        builder = builder.with_initialization_script(&init_script);
    }
    if debug {
        builder = builder.with_devtools(true);
    }
    // Delay the first navigation until WebKitGTK settings have been applied.
    builder = builder.with_url("about:blank");

    let webview = build_webview(builder, &window)?;
    webview.load_url(&config.url).map_err(|e| e.to_string())?;
    Config::mark_app_running(&app_id, std::process::id());

    let app_id_clone = app_id.clone();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + Duration::from_secs(2));

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested | tao::event::WindowEvent::Destroyed,
                ..
            }
            | tao::event::Event::LoopDestroyed => {
                Config::mark_app_stopped(&app_id_clone);
                *control_flow = ControlFlow::Exit;
            }
            _ => {
                if now_seconds().saturating_sub(heartbeat.load(Ordering::Relaxed)) > 12 {
                    eprintln!("[WebFlow] WebView heartbeat stopped for '{}', closing app", app_id_clone);
                    Config::mark_app_stopped(&app_id_clone);
                    std::process::exit(1);
                }
            }
        }
    });
}

fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
