pub mod custom;
pub mod system;

use tao::event_loop::EventLoopWindowTarget;
use tao::window::{Icon, Window};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use wry::{WebView, WebViewBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFrameStyle {
    System,
    Custom,
}

pub const APP_MIN_WIDTH: u32 = 700;
pub const APP_MIN_HEIGHT: u32 = 480;
pub const MANAGER_MIN_WIDTH: u32 = 850;
pub const MANAGER_MIN_HEIGHT: u32 = 480;
pub const MAX_WINDOW_WIDTH: u32 = 7680;
pub const MAX_WINDOW_HEIGHT: u32 = 4320;

#[allow(dead_code)]
pub struct WindowOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub frame_style: WindowFrameStyle,
    pub debug: bool,
    pub position: Option<(i32, i32)>,
    pub icon: Option<Icon>,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
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
            icon: None,
            min_width: APP_MIN_WIDTH,
            min_height: APP_MIN_HEIGHT,
            max_width: MAX_WINDOW_WIDTH,
            max_height: MAX_WINDOW_HEIGHT,
        }
    }
}

pub fn load_window_icon(path: &Path) -> Option<Icon> {
    let file = File::open(path).ok()?;
    load_window_icon_from_reader(BufReader::new(file))
}

pub fn load_window_icon_bytes(bytes: &[u8]) -> Option<Icon> {
    load_window_icon_from_reader(BufReader::new(std::io::Cursor::new(bytes)))
}

fn load_window_icon_from_reader<R: std::io::Read + std::io::Seek>(reader: R) -> Option<Icon> {
    let mut decoder = png::Decoder::new(reader);
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().ok()?;
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).ok()?;
    let data = &buffer[..info.buffer_size()];

    let rgba = match info.color_type {
        png::ColorType::Rgba => data.to_vec(),
        png::ColorType::Rgb => data
            .chunks_exact(3)
            .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 255])
            .collect(),
        png::ColorType::Grayscale => data
            .iter()
            .flat_map(|&value| [value, value, value, 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => data
            .chunks_exact(2)
            .flat_map(|pixel| [pixel[0], pixel[0], pixel[0], pixel[1]])
            .collect(),
        _ => return None,
    };

    Icon::from_rgba(rgba, info.width, info.height).ok()
}

pub fn apply_window_icon(window: &Window, _path: &Path, _icon: Option<Icon>) {
    #[cfg(target_os = "linux")]
    {
        use gtk::prelude::*;
        use tao::platform::unix::WindowExtUnix;

        if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_file(_path) {
            window.gtk_window().set_icon(Some(&pixbuf));
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        window.set_window_icon(_icon);
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
