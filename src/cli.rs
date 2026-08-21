use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "webflow-runtime",
    author = "WebFlow Team",
    version = "0.1.0",
    about = "Ultra-lightweight engine to turn websites into desktop applications"
)]
pub struct CliArgs {
    /// App ID to launch directly without loading Manager UI
    #[arg(short, long)]
    pub app: Option<String>,

    /// Enable debug mode and webview developer tools
    #[arg(short, long)]
    pub debug: bool,

    /// Include background polling and high-volume debug events
    #[arg(long)]
    pub debug_verbose: bool,

    /// Mark a manager launch initiated by the operating system autostart entry
    #[arg(long, hide = true)]
    pub autostart: bool,

    /// Create application from template ID
    #[arg(long, value_name = "TEMPLATE_ID")]
    pub create_from_template: Option<String>,

    /// List all available application templates
    #[arg(long)]
    pub list_templates: bool,
}
