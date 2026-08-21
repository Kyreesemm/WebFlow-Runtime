#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app_runner;
mod cli;
mod config;
mod debug;
mod instance;
mod manager;
mod tray;
mod window;

use clap::Parser;
use cli::CliArgs;
use config::{AppConfig, Config};

fn main() {
    // Fix for WebKitGTK rendering issues on Wayland/X11 with GBM buffers
    #[cfg(target_os = "linux")]
    {
        // Disable DMA-BUF renderer that causes GBM buffer creation failures
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }

        // Some WebKitGTK/GPU combinations terminate the web process when
        // video playback starts. Software compositing keeps the app process
        // alive and can still play video, with a small rendering trade-off.
        if std::env::var("WEBKIT_DISABLE_COMPOSITING_MODE").is_err() {
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        }
        
        // Force native Wayland backend (remove X11 fallback)
        if std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "wayland");
        }
    }

    let args = CliArgs::parse();
    let debug_enabled = args.debug || args.debug_file;

    if debug_enabled {
        if let Err(error) = debug::initialize(args.debug, args.debug_file) {
            eprintln!("Failed to initialize debug logging: {error}");
        }
    }

    #[cfg(target_os = "windows")]
    if args.debug {
        unsafe {
            use windows_sys::Win32::System::Console::{
                AttachConsole, GetConsoleMode, GetStdHandle, SetConsoleMode,
                ATTACH_PARENT_PROCESS, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
                STD_ERROR_HANDLE, STD_OUTPUT_HANDLE,
            };

            let _ = AttachConsole(ATTACH_PARENT_PROCESS);
            for standard_handle in [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
                let handle = GetStdHandle(standard_handle);
                let mut mode = 0;
                if GetConsoleMode(handle, &mut mode) != 0 {
                    let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
                }
            }
        }
    }

    // Pure CLI mode: List templates
    if args.list_templates {
        println!("Available templates / Доступные шаблоны:");
        println!("  claude      - Anthropic Claude Assistant");
        println!("  chatgpt     - OpenAI ChatGPT");
        println!("  deepseek    - DeepSeek AI Chat");
        println!("  youtube     - YouTube Video Streaming");

        let tmpl_dir = Config::get_base_dir().join("templates");
        if tmpl_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(tmpl_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() && p.join("config.json").exists() {
                        if let Ok(content) = std::fs::read_to_string(p.join("config.json")) {
                            if let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&content) {
                                let desc = cfg.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                if let Some(id) = p.file_name().and_then(|s| s.to_str()) {
                                    println!("  {:11} - {}", id, desc);
                                }
                            }
                        }
                    }
                }
            }
        }
        std::process::exit(0);
    }

    // Pure CLI mode: Create application from template
    if let Some(template_id) = args.create_from_template {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let app_id = format!("{}-{}", template_id, now);

        let (name, url) = match template_id.as_str() {
            "claude" => ("Claude AI", "https://claude.ai"),
            "chatgpt" => ("ChatGPT", "https://chatgpt.com"),
            "deepseek" => ("DeepSeek", "https://chat.deepseek.com"),
            "youtube" => ("YouTube", "https://youtube.com"),
            _ => ("New App", "https://example.com"),
        };

        let mut config = AppConfig::default();
        config.name = name.to_string();
        config.url = url.to_string();
        config.window.title = name.to_string();

        if let Err(e) = Config::save_app_config(&app_id, &config) {
            eprintln!("Failed to create app: {}", e);
            std::process::exit(1);
        }

        println!("✓ Application '{}' created!", name);
        println!("  ID: {}", app_id);
        println!("  Path: {}", Config::get_apps_dir().join(&app_id).display());
        println!("\nLaunch: webflow-runtime --app {}", app_id);
        std::process::exit(0);
    }

    // App mode: Launch specific app directly (ZERO Manager UI loaded into memory!)
    if let Some(app_id) = args.app {
        if let Err(e) = app_runner::run_app(app_id, args.debug) {
            eprintln!("Error launching app: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Default mode: Launch Manager Web UI
    if let Err(e) = manager::run_manager(debug_enabled, args.debug_verbose, args.autostart) {
        eprintln!("Error launching manager: {}", e);
        std::process::exit(1);
    }
}
