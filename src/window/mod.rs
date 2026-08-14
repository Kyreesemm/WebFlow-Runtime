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
    pub position: Option<(i32, i32)>,
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
            position: None,
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
        let webview = builder.build_gtk(vbox).map_err(|e| e.to_string())?;
        configure_webkit_settings(&vbox);
        Ok(webview)
    }

    #[cfg(not(target_os = "linux"))]
    {
        builder.build(window).map_err(|e| e.to_string())
    }
}

#[cfg(target_os = "linux")]
fn configure_webkit_settings(vbox: &gtk::Box) {
    use gtk::prelude::*;
    use webkit2gtk::{SettingsExt, WebViewExt};

    for child in vbox.children() {
        if let Ok(webview) = child.downcast::<webkit2gtk::WebView>() {
            if let Some(settings) = WebViewExt::settings(&webview) {
                settings.set_enable_media(true);
                settings.set_enable_media_capabilities(true);
                settings.set_enable_media_stream(true);
                settings.set_enable_mediasource(true);
                settings.set_enable_webaudio(true);
                settings.set_enable_webgl(true);

                // Keep video rendering on the software path. This avoids the
                // WebKitGTK GPU crash seen when YouTube starts playback.
                settings.set_hardware_acceleration_policy(
                    webkit2gtk::HardwareAccelerationPolicy::Never,
                );
            }

        }
    }
}
