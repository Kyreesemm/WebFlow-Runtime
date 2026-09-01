#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app_runner;
mod cli;
mod config;
mod debug;
mod instance;
mod manager;
mod updater;
mod tray;
mod window;

use clap::Parser;
use cli::CliArgs;
use config::Config;

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
    if let Some(archive) = args.update_helper.as_deref() {
        if let Err(error) = updater::run_update_helper(archive, args.update_parent_pid) {
            eprintln!("Update installation failed: {error}");
            std::process::exit(1);
        }
        return;
    }
    if args.restart {
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    let manager_mode = args.app.is_none()
        && !args.list_templates
        && args.create_from_template.is_none();
    let persisted_manager_file_logging = manager_mode
        && config::Config::load_engine_settings().manager_log_to_file;
    let file_logging = args.debug_file || persisted_manager_file_logging;
    let debug_enabled = args.debug || file_logging;
    let log_prefix = if args.app.is_some()
        && !args.list_templates
        && args.create_from_template.is_none()
    {
        "app-session"
    } else {
        "session"
    };

    if debug_enabled {
        if let Err(error) = debug::initialize(args.debug, file_logging, log_prefix) {
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
        for template in manager::ipc::list_templates() {
            let description = template.description.get("en").and_then(|value| value.as_str()).unwrap_or("");
            println!("  {:11} - {}", template.id, description);
        }
        std::process::exit(0);
    }

    /*
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
        std::process::exit(0); */

    // Pure CLI mode: Create application from template
    if let Some(template_id) = args.create_from_template {
        let app_id = match manager::ipc::create_template_app(&template_id) {
            Ok(app_id) => app_id,
            Err(e) => {
                eprintln!("Failed to create app: {}", e);
                std::process::exit(1);
            }
        };
        println!("✓ Application '{}' created!", template_id);
        println!("  ID: {}", app_id);
        println!("  Path: {}", Config::get_apps_dir().join(&app_id).display());
        println!("\nLaunch: webflow-runtime --app {}", app_id);
        std::process::exit(0);
    }

    // App mode: Launch specific app directly (ZERO Manager UI loaded into memory!)
    if let Some(app_id) = args.app {
        if let Err(e) = app_runner::run_app(app_id, debug_enabled) {
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
