pub mod bridge_js;
pub mod ipc;
pub mod webui;

use crate::window::{build_webview, WindowFactory, WindowFrameStyle, WindowOptions};
use std::sync::{Arc, Mutex};
use tao::event_loop::{ControlFlow, EventLoop};
use wry::{WebView, WebViewBuilder};

pub fn run_manager(debug: bool) -> Result<(), String> {
    let event_loop = EventLoop::new();

    let options = WindowOptions {
        title: "WebFlow Runtime Manager".to_string(),
        width: 1000,
        height: 700,
        resizable: true,
        frame_style: WindowFrameStyle::System,
        debug,
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
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
