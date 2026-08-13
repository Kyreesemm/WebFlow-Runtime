use crate::config::{AppConfig, Config};
use crate::window::{build_webview, WindowFactory, WindowFrameStyle, WindowOptions};
use tao::event_loop::{ControlFlow, EventLoop};
use wry::{WebContext, WebViewBuilder};

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
    };

    let window = factory.create_window(&event_loop, &options)?;

    // Isolated or shared storage directory for WebContext
    let storage_dir = if config.isolated_storage {
        Config::get_apps_dir().join(&app_id).join("storage")
    } else {
        Config::get_shared_storage_dir()
    };
    std::fs::create_dir_all(&storage_dir).map_err(|e| e.to_string())?;

    let mut web_context = WebContext::new(Some(storage_dir));
    let mut builder = WebViewBuilder::new_with_web_context(&mut web_context);

    // User Agent
    if config.user_agent == "custom" {
        if let Some(ref ua) = config.custom_user_agent {
            if !ua.is_empty() {
                builder = builder.with_user_agent(ua);
            }
        }
    }

    // Custom CSS & JS via initialization scripts
    let mut init_script = String::new();

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

    builder = builder.with_url(&config.url);

    let _webview = build_webview(builder, &window)?;

    // Mark app as running
    let pid = std::process::id();
    Config::mark_app_running(&app_id, pid);

    let app_id_clone = app_id.clone();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => {
                Config::mark_app_stopped(&app_id_clone);
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
