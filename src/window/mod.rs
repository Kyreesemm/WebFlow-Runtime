pub mod custom;
pub mod system;

use tao::event_loop::EventLoopWindowTarget;
use tao::window::Window;
use wry::{WebView, WebViewBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFrameStyle {
    System,
    Custom,
}

#[allow(dead_code)]
pub struct WindowOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub frame_style: WindowFrameStyle,
    pub debug: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "WebFlow Application".to_string(),
            width: 1024,
            height: 768,
            resizable: true,
            frame_style: WindowFrameStyle::System,
            debug: false,
        }
    }
}

pub enum WindowFactory {
    System(system::SystemWindowFactory),
    Custom(custom::CustomWindowFactory),
}

impl WindowFactory {
    pub fn new(style: WindowFrameStyle) -> Self {
        match style {
            WindowFrameStyle::System => WindowFactory::System(system::SystemWindowFactory),
            WindowFrameStyle::Custom => WindowFactory::Custom(custom::CustomWindowFactory),
        }
    }

    pub fn create_window<T: 'static>(
        &self,
        event_loop: &EventLoopWindowTarget<T>,
        options: &WindowOptions,
    ) -> Result<Window, String> {
        match self {
            WindowFactory::System(f) => f.build(event_loop, options),
            WindowFactory::Custom(f) => f.build(event_loop, options),
        }
    }
}

pub fn build_webview(builder: WebViewBuilder<'_>, window: &Window) -> Result<WebView, String> {
    #[cfg(target_os = "linux")]
    {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window
            .default_vbox()
            .ok_or_else(|| "Failed to get GTK container vbox from window".to_string())?;
        builder.build_gtk(vbox).map_err(|e| e.to_string())
    }

    #[cfg(not(target_os = "linux"))]
    {
        builder.build(window).map_err(|e| e.to_string())
    }
}
