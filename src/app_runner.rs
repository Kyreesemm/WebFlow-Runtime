use crate::config::{AppConfig, Config};
use crate::window::{
    apply_window_icon, build_webview, load_window_icon, WindowFactory, WindowFrameStyle,
    WindowOptions, APP_MIN_HEIGHT, APP_MIN_WIDTH, MAX_WINDOW_HEIGHT, MAX_WINDOW_WIDTH,
};
use tao::event_loop::{ControlFlow, EventLoop};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wry::{WebContext, WebViewBuilder};

#[cfg(target_os = "linux")]
fn prepare_linux_app_id(app_id: &str, config: &AppConfig, icon_path: &std::path::Path) -> String {
    use std::fs;
    use std::path::PathBuf;

    let safe_id: String = app_id
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '-' })
        .collect();
    let desktop_id = format!("com.webflow.runtime.{}", safe_id);

    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));

    if let Some(data_home) = data_home {
        let applications_dir = data_home.join("applications");
        let _ = fs::create_dir_all(&applications_dir);
        if let Ok(executable) = std::env::current_exe() {
            let escape_exec_arg = |value: &str| {
                format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
            };
            let desktop_entry = format!(
                "[Desktop Entry]\nType=Application\nName={}\nExec={} --app {}\nIcon={}\nNoDisplay=true\nTerminal=false\nStartupNotify=true\n",
                config.name.replace(['\n', '\r'], " "),
                escape_exec_arg(&executable.to_string_lossy()),
                escape_exec_arg(app_id),
                icon_path.to_string_lossy()
            );
            let _ = fs::write(
                applications_dir.join(format!("{}.desktop", desktop_id)),
                desktop_entry,
            );
        }
    }

    desktop_id
}

#[cfg(target_os = "linux")]
fn create_app_event_loop(app_id: &str, config: &AppConfig, icon_path: &std::path::Path) -> EventLoop<()> {
    use tao::event_loop::EventLoopBuilder;
    use tao::platform::unix::EventLoopBuilderExtUnix;

    let desktop_id = prepare_linux_app_id(app_id, config, icon_path);
    let mut builder = EventLoopBuilder::new();
    builder.with_app_id(desktop_id);
    builder.build()
}

#[cfg(not(target_os = "linux"))]
fn create_app_event_loop(_app_id: &str, _config: &AppConfig, _icon_path: &std::path::Path) -> EventLoop<()> {
    EventLoop::new()
}

#[cfg(target_os = "linux")]
fn configure_linux_window_backend() {
    let is_wayland_session = std::env::var("XDG_SESSION_TYPE")
        .map(|value| value.eq_ignore_ascii_case("wayland"))
        .unwrap_or(false);
    let has_xwayland = std::env::var_os("DISPLAY").is_some();

    // GTK3/Tao cannot send the staged per-toplevel-icon Wayland protocol.
    // KDE therefore ignores GtkWindow::icon and shows the generic Wayland
    // application icon. Use XWayland for app windows when it is available;
    // the X11 window icon path is supported by GTK and KDE.
    if is_wayland_session && has_xwayland {
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
}

#[cfg(not(target_os = "linux"))]
fn configure_linux_window_backend() {}

const LINUX_CHROME_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const WINDOWS_CHROME_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
const IPHONE_SAFARI_UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Mobile/15E148 Safari/604.1";
const ANDROID_CHROME_UA: &str = "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.200 Mobile Safari/537.36";
const CHROME_LINUX_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36";
const CHROME_WINDOWS_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36";

pub fn run_app(app_id: String, debug: bool) -> Result<(), String> {
    let config: AppConfig = Config::load_app_config(&app_id)
        .ok_or_else(|| format!("App configuration not found for '{}'", app_id))?;

    configure_linux_window_backend();
    let icon_path = Config::get_app_icon_path(&app_id);
    let event_loop = create_app_event_loop(&app_id, &config, &icon_path);
    let frame_style = if config.window.custom_frame {
        WindowFrameStyle::Custom
    } else {
        WindowFrameStyle::System
    };
    let factory = WindowFactory::new(frame_style);
    let options = WindowOptions {
        title: config.window.title.clone(),
        width: config
            .window
            .width
            .clamp(APP_MIN_WIDTH, MAX_WINDOW_WIDTH),
        height: config
            .window
            .height
            .clamp(APP_MIN_HEIGHT, MAX_WINDOW_HEIGHT),
        resizable: config.window.resizable,
        frame_style,
        debug,
        position: None,
        icon: load_window_icon(&icon_path),
        min_width: APP_MIN_WIDTH,
        min_height: APP_MIN_HEIGHT,
        max_width: MAX_WINDOW_WIDTH,
        max_height: MAX_WINDOW_HEIGHT,
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
    apply_window_icon(&window, &icon_path, options.icon.clone());
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
