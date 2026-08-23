use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "webflow-runtime",
    author = "KRM Tech Software",
    version,
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

    /// Write a detailed debug session log to the executable directory
    #[arg(long)]
    pub debug_file: bool,

    /// Mark a manager launch initiated by the operating system autostart entry
    #[arg(long, hide = true)]
    pub autostart: bool,

    /// Delay an internal manager restart until the previous process exits
    #[arg(long, hide = true)]
    pub restart: bool,

    /// Internal updater helper mode
    #[arg(long, hide = true, value_name = "ARCHIVE")]
    pub update_helper: Option<String>,

    /// PID of the Manager process that must exit before installation
    #[arg(long, hide = true, value_name = "PID")]
    pub update_parent_pid: Option<u32>,

    /// Create application from template ID
    #[arg(long, value_name = "TEMPLATE_ID")]
    pub create_from_template: Option<String>,

    /// List all available application templates
    #[arg(long)]
    pub list_templates: bool,
}
