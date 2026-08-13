use tao::event_loop::EventLoop;
use tao::platform::unix::WindowExtUnix;
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let gtk_window = window.gtk_window();
    println!("GTK window obtained: {:?}", gtk_window);

    let webview = WebViewBuilder::new().build(gtk_window);
    match webview {
        Ok(_) => println!("WebView created successfully!"),
        Err(e) => println!("Error creating WebView: {:?}", e),
    }
}
