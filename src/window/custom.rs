use super::WindowOptions;
use tao::dpi::{LogicalSize, PhysicalPosition};
use tao::event_loop::EventLoopWindowTarget;
use tao::window::{Window, WindowBuilder};

pub struct CustomWindowFactory;

impl CustomWindowFactory {
    pub fn build<T: 'static>(
        &self,
        event_loop: &EventLoopWindowTarget<T>,
        options: &WindowOptions,
    ) -> Result<Window, String> {
        let mut builder = WindowBuilder::new()
            .with_title(&options.title)
            .with_inner_size(LogicalSize::new(options.width, options.height))
            .with_min_inner_size(LogicalSize::new(options.min_width, options.min_height))
            .with_max_inner_size(LogicalSize::new(options.max_width, options.max_height))
            .with_resizable(options.resizable)
            .with_decorations(false)
            .with_transparent(true)
            .with_window_icon(options.icon.clone());

        if let Some((x, y)) = options.position {
            builder = builder.with_position(PhysicalPosition::new(x, y));
        }

        builder.build(event_loop).map_err(|e| e.to_string())
    }
}
