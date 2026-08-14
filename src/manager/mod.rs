pub mod bridge_js;
pub mod ipc;
pub mod webui;

use crate::tray::{ManagerTray, EXIT_ID, SHOW_ID};
use crate::window::{
    apply_window_icon, build_webview, load_window_icon_bytes, WindowFactory, WindowFrameStyle,
    WindowOptions,
};
use serde_json::{json, Value};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tao::event_loop::ControlFlow;
use tao::window::Window;
use tray_icon::menu::MenuEvent;
use wry::{WebView, WebViewBuilder};

const MANAGER_ICON_BYTES: &[u8] = include_bytes!("../../materials/default/webflow_runtime_icon.png");
#[cfg(target_os = "linux")]
const MANAGER_APP_ID: &str = "com.webflow.runtime.manager";

#[cfg(target_os = "linux")]
fn prepare_manager_desktop_entry(icon_path: &std::path::Path) {
    use std::fs;
    use std::path::PathBuf;

    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    let Some(data_home) = data_home else {
        return;
    };

    let applications_dir = data_home.join("applications");
    let _ = fs::create_dir_all(&applications_dir);
    let Ok(executable) = std::env::current_exe() else {
        return;
    };
    let escape = |value: &str| value.replace('\\', "\\\\").replace('"', "\\\"");
    let desktop_entry = format!(
        "[Desktop Entry]\nType=Application\nName=WebFlow Runtime Manager\nExec=\"{}\"\nIcon={}\nNoDisplay=true\nTerminal=false\nStartupNotify=true\n",
        escape(&executable.to_string_lossy()),
        icon_path.to_string_lossy()
    );
    let _ = fs::write(
        applications_dir.join(format!("{}.desktop", MANAGER_APP_ID)),
        desktop_entry,
    );
}

#[cfg(target_os = "linux")]
fn configure_manager_window_backend() {
    let is_wayland_session = std::env::var("XDG_SESSION_TYPE")
        .map(|value| value.eq_ignore_ascii_case("wayland"))
        .unwrap_or(false);
    let has_xwayland = std::env::var_os("DISPLAY").is_some();

    // GTK3 cannot reliably provide per-window icons through native Wayland.
    // Use XWayland when available, matching the already working app-window
    // path; retain native Wayland as a fallback for systems without XWayland.
    if is_wayland_session && has_xwayland {
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
}

fn load_manager_geometry() -> (u32, u32, Option<(i32, i32)>) {
    let path = crate::config::Config::get_config_dir().join("window_state.json");
    let state = fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<Value>(&contents).ok())
        .unwrap_or_else(|| json!({}));

    let width = state
        .get("width")
        .and_then(Value::as_u64)
        .map(|value| value.clamp(800, 7680) as u32)
        .unwrap_or(1000);
    let height = state
        .get("height")
        .and_then(Value::as_u64)
        .map(|value| value.clamp(480, 4320) as u32)
        .unwrap_or(700);
    let position = state
        .get("x")
        .and_then(Value::as_i64)
        .zip(state.get("y").and_then(Value::as_i64))
        .map(|(x, y)| (x as i32, y as i32));

    (width, height, position)
}

fn save_manager_geometry(window: &Window) {
    let path = crate::config::Config::get_config_dir().join("window_state.json");
    let mut state = fs::read_to_string(&path)
        .ok()
        .and_then(|contents| serde_json::from_str::<Value>(&contents).ok())
        .unwrap_or_else(|| json!({}));

    if !state.is_object() {
        state = json!({});
    }

    if let Some(state) = state.as_object_mut() {
        let size = window.inner_size();
        state.insert("width".to_string(), json!(size.width));
        state.insert("height".to_string(), json!(size.height));

        if let Ok(position) = window.outer_position() {
            state.insert("x".to_string(), json!(position.x));
            state.insert("y".to_string(), json!(position.y));
        }

        if let Ok(contents) = serde_json::to_string_pretty(&state) {
            let _ = fs::write(path, contents);
        }
    }
}

pub fn run_manager(debug: bool) -> Result<(), String> {
    let (width, height, position) = load_manager_geometry();
    let icon_path = crate::config::Config::get_base_dir()
        .join("materials")
        .join("default")
        .join("webflow_runtime_icon.png");

    #[cfg(target_os = "linux")]
    prepare_manager_desktop_entry(&icon_path);

    #[cfg(target_os = "linux")]
    configure_manager_window_backend();

    #[cfg(target_os = "linux")]
    let event_loop = {
        use tao::event_loop::EventLoopBuilder;
        use tao::platform::unix::EventLoopBuilderExtUnix;

        let mut builder = EventLoopBuilder::new();
        builder.with_app_id(MANAGER_APP_ID);
        builder.build()
    };
    #[cfg(not(target_os = "linux"))]
    let event_loop = tao::event_loop::EventLoop::new();

    let options = WindowOptions {
        title: "WebFlow Runtime Manager".to_string(),
        width,
        height,
        resizable: true,
        frame_style: WindowFrameStyle::System,
        debug,
        position,
        icon: load_window_icon_bytes(MANAGER_ICON_BYTES),
    };

    let factory = WindowFactory::new(WindowFrameStyle::System);
    let window = factory.create_window(&event_loop, &options)?;
    apply_window_icon(&window, &icon_path, options.icon.clone());

    let webview_handle: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));
    create_webview(&window, webview_handle.clone(), debug)?;

    let mut tray_enabled = crate::config::Config::load_engine_settings().minimize_to_tray;
    let mut tray = if tray_enabled {
        match ManagerTray::create(MANAGER_ICON_BYTES) {
            Ok(new_tray) => {
                new_tray.set_interface_visible(true);
                Some(new_tray)
            }
            Err(error) => {
                eprintln!("Failed to create manager tray icon: {error}");
                tray_enabled = false;
                None
            }
        }
    } else {
        None
    };
    let menu_events = MenuEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(250));

        while let Ok(menu_event) = menu_events.try_recv() {
            if let Some(current_tray) = tray.as_ref() {
                if menu_event.id() == current_tray.show_item.id() || menu_event.id().0 == SHOW_ID {
                    if window.is_visible() {
                        save_manager_geometry(&window);
                        window.set_visible(false);
                        *webview_handle.lock().unwrap() = None;
                        current_tray.set_interface_visible(false);
                    } else {
                        window.set_visible(true);
                        window.set_focus();
                        if webview_handle.lock().unwrap().is_none() {
                            if let Err(error) = create_webview(&window, webview_handle.clone(), debug) {
                                eprintln!("Failed to restore manager WebView: {error}");
                            }
                        }
                        current_tray.set_interface_visible(true);
                    }
                } else if menu_event.id() == current_tray.exit_item.id() || menu_event.id().0 == EXIT_ID {
                    *webview_handle.lock().unwrap() = None;
                    save_manager_geometry(&window);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        }

        let desired_tray = crate::config::Config::load_engine_settings().minimize_to_tray;
        if desired_tray != tray_enabled {
            tray_enabled = desired_tray;
            if tray_enabled {
                match ManagerTray::create(MANAGER_ICON_BYTES) {
                    Ok(new_tray) => {
                        new_tray.set_interface_visible(window.is_visible());
                        tray = Some(new_tray);
                    }
                    Err(error) => {
                        eprintln!("Failed to create manager tray icon: {error}");
                        tray_enabled = false;
                    }
                }
            } else {
                tray = None;
            }
        }

        if tray_enabled && window.is_minimized() {
            save_manager_geometry(&window);
            window.set_visible(false);
            window.set_minimized(false);
            *webview_handle.lock().unwrap() = None;
            if let Some(current_tray) = tray.as_ref() {
                current_tray.set_interface_visible(false);
            }
        }

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested
                    | tao::event::WindowEvent::Destroyed,
                ..
            } => {
                if tray_enabled {
                    save_manager_geometry(&window);
                    window.set_visible(false);
                    *webview_handle.lock().unwrap() = None;
                    if let Some(current_tray) = tray.as_ref() {
                        current_tray.set_interface_visible(false);
                    }
                } else {
                    save_manager_geometry(&window);
                    *webview_handle.lock().unwrap() = None;
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}

fn create_webview(
    window: &Window,
    webview_handle: Arc<Mutex<Option<WebView>>>,
    debug: bool,
) -> Result<(), String> {
    let protocol = WebViewBuilder::new().with_custom_protocol("webflow".into(), move |_webview, req| {
        webui::handle_custom_protocol_request(req.uri().path())
    });
    let webview_clone = webview_handle.clone();
    let mut builder = protocol
        .with_initialization_script(bridge_js::INJECTED_BRIDGE_JS)
        .with_ipc_handler(move |req| {
            ipc::handle_ipc_message(webview_clone.clone(), req.body());
        })
        .with_url("webflow://manager/index.html");

    if debug {
        builder = builder.with_devtools(true);
    }

    let webview = build_webview(builder, window)?;
    *webview_handle.lock().unwrap() = Some(webview);
    Ok(())
}
