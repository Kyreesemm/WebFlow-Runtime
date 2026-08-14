pub mod bridge_js;
pub mod ipc;
pub mod webui;

use crate::window::{build_webview, WindowFactory, WindowFrameStyle, WindowOptions};
use serde_json::{json, Value};
use std::fs;
use std::sync::{Arc, Mutex};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::Window;
use wry::{WebView, WebViewBuilder};

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
    let event_loop = EventLoop::new();
    let (width, height, position) = load_manager_geometry();

    let options = WindowOptions {
        title: "WebFlow Runtime Manager".to_string(),
        width,
        height,
        resizable: true,
        frame_style: WindowFrameStyle::System,
        debug,
        position,
    };

    let factory = WindowFactory::new(WindowFrameStyle::System);
    let window = factory.create_window(&event_loop, &options)?;

    let webview_handle: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));
    let webview_clone = webview_handle.clone();

    let mut builder = WebViewBuilder::new()
        .with_custom_protocol("webflow".into(), move |_webview, req| {
            webui::handle_custom_protocol_request(req.uri().path())
        })
        .with_initialization_script(bridge_js::INJECTED_BRIDGE_JS)
        .with_ipc_handler(move |req| {
            ipc::handle_ipc_message(webview_clone.clone(), req.body());
        })
        .with_url("webflow://manager/index.html");

    if debug {
        builder = builder.with_devtools(true);
    }

    let webview = build_webview(builder, &window)?;

    *webview_handle.lock().unwrap() = Some(webview);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested
                    | tao::event::WindowEvent::Destroyed,
                ..
            } => {
                save_manager_geometry(&window);
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
