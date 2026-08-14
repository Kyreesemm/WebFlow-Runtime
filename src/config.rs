use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    #[serde(default = "default_true")]
    pub resizable: bool,
    #[serde(default)]
    pub custom_frame: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "New Application".to_string(),
            width: 1024,
            height: 768,
            resizable: true,
            custom_frame: false,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default)]
    pub custom_user_agent: Option<String>,
    #[serde(default)]
    pub custom_scrollbar: bool,
    #[serde(default = "default_true")]
    pub isolated_storage: bool,
    #[serde(default)]
    pub custom_css: Option<String>,
    #[serde(default)]
    pub custom_js: Option<String>,
    #[serde(default)]
    pub imported_cookies: Vec<serde_json::Value>,
}

fn default_user_agent() -> String {
    "default".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "New App".to_string(),
            url: "https://example.com".to_string(),
            icon: None,
            window: WindowConfig::default(),
            user_agent: "default".to_string(),
            custom_user_agent: None,
            custom_scrollbar: false,
            isolated_storage: true,
            custom_css: None,
            custom_js: None,
            imported_cookies: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSettings {
    #[serde(default)]
    pub userdata_path: Option<String>,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default)]
    pub minimize_to_tray: bool,
    #[serde(default = "default_true")]
    pub app_tray_icons: bool,
    #[serde(default)]
    pub tray_apps_menu: bool,
    #[serde(default = "default_true")]
    pub google_oauth_fallback: bool,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            userdata_path: None,
            autostart: false,
            minimize_to_tray: false,
            app_tray_icons: true,
            tray_apps_menu: false,
            google_oauth_fallback: true,
        }
    }
}

pub struct Config;

impl Config {
    pub fn get_base_dir() -> PathBuf {
        // Match the former Python layout while developing: `cargo run` puts
        // the executable under target/debug, but userdata remains alongside
        // Cargo.toml in the project root. Release builds keep userdata next
        // to the installed executable.
        #[cfg(debug_assertions)]
        {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        }

        #[cfg(not(debug_assertions))]
        {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(parent) = exe_path.parent() {
                    return parent.to_path_buf();
                }
            }
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        }
    }

    pub fn get_userdata_base() -> PathBuf {
        // Check engine settings for custom userdata_path
        let config_dir = Self::get_base_dir().join("userdata").join("config");
        let settings_file = config_dir.join("engine_settings.json");
        if settings_file.exists() {
            if let Ok(content) = fs::read_to_string(&settings_file) {
                if let Ok(st) = serde_json::from_str::<EngineSettings>(&content) {
                    if let Some(ref path) = st.userdata_path {
                        let p = PathBuf::from(path);
                        if p.exists() || fs::create_dir_all(&p).is_ok() {
                            return p;
                        }
                    }
                }
            }
        }

        let base = Self::get_base_dir().join("userdata");
        let _ = fs::create_dir_all(&base);
        base
    }

    pub fn get_apps_dir() -> PathBuf {
        let dir = Self::get_userdata_base().join("apps");
        let _ = fs::create_dir_all(&dir);
        dir
    }

    pub fn get_config_dir() -> PathBuf {
        let dir = Self::get_userdata_base().join("config");
        let _ = fs::create_dir_all(&dir);
        dir
    }

    pub fn get_runtime_dir() -> PathBuf {
        let dir = Self::get_userdata_base().join("runtime");
        let _ = fs::create_dir_all(&dir);
        dir
    }

    pub fn get_shared_storage_dir() -> PathBuf {
        let dir = Self::get_userdata_base().join("shared_storage");
        let _ = fs::create_dir_all(&dir);
        dir
    }

    pub fn get_app_icon_path(app_id: &str) -> PathBuf {
        let custom_icon = Self::get_apps_dir().join(app_id).join("icon.png");
        if custom_icon.exists() {
            custom_icon
        } else {
            Self::get_base_dir()
                .join("materials")
                .join("default")
                .join("app_icon.png")
        }
    }

    pub fn ensure_app_icon(app_id: &str) {
        let app_dir = Self::get_apps_dir().join(app_id);
        let icon_path = app_dir.join("icon.png");
        if icon_path.exists() {
            return;
        }

        let default_icon = Self::get_base_dir()
            .join("materials")
            .join("default")
            .join("app_icon.png");
        if default_icon.exists() {
            let _ = fs::create_dir_all(&app_dir);
            let _ = fs::copy(default_icon, icon_path);
        }
    }

    pub fn list_apps() -> Vec<String> {
        let apps_dir = Self::get_apps_dir();
        let mut apps = Vec::new();
        if let Ok(entries) = fs::read_dir(apps_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("config.json").exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        apps.push(name.to_string());
                    }
                }
            }
        }
        apps.sort();
        apps
    }

    pub fn load_app_config(app_id: &str) -> Option<AppConfig> {
        let path = Self::get_apps_dir().join(app_id).join("config.json");
        if !path.exists() {
            return None;
        }
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save_app_config(app_id: &str, config: &AppConfig) -> Result<(), String> {
        let app_dir = Self::get_apps_dir().join(app_id);
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
        let path = app_dir.join("config.json");
        let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Self::ensure_app_icon(app_id);
        Ok(())
    }

    pub fn delete_app(app_id: &str) -> Result<(), String> {
        let app_dir = Self::get_apps_dir().join(app_id);
        if app_dir.exists() {
            fs::remove_dir_all(app_dir).map_err(|e| e.to_string())?;
        }
        let pid_file = Self::get_runtime_dir().join(format!("{}.pid", app_id));
        if pid_file.exists() {
            let _ = fs::remove_file(pid_file);
        }
        Ok(())
    }

    pub fn load_engine_settings() -> EngineSettings {
        let path = Self::get_config_dir().join("engine_settings.json");
        if !path.exists() {
            return EngineSettings::default();
        }
        fs::read_to_string(path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    }

    pub fn save_engine_settings(settings: &EngineSettings) -> Result<(), String> {
        let path = Self::get_config_dir().join("engine_settings.json");
        let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    // PID Tracking
    pub fn mark_app_running(app_id: &str, pid: u32) {
        let file = Self::get_runtime_dir().join(format!("{}.pid", app_id));
        let _ = fs::write(file, pid.to_string());
    }

    pub fn mark_app_stopped(app_id: &str) {
        let file = Self::get_runtime_dir().join(format!("{}.pid", app_id));
        if file.exists() {
            let _ = fs::remove_file(file);
        }
    }

    pub fn is_app_running(app_id: &str) -> bool {
        let file = Self::get_runtime_dir().join(format!("{}.pid", app_id));
        if !file.exists() {
            return false;
        }
        if let Ok(content) = fs::read_to_string(&file) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                if Self::pid_exists(pid) {
                    return true;
                }
            }
        }
        let _ = fs::remove_file(file);
        false
    }

    pub fn get_running_apps() -> Vec<String> {
        let dir = Self::get_runtime_dir();
        let mut running = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pid") {
                    if let Some(app_id) = path.file_stem().and_then(|s| s.to_str()) {
                        if Self::is_app_running(app_id) {
                            running.push(app_id.to_string());
                        }
                    }
                }
            }
        }
        running
    }

    fn pid_exists(pid: u32) -> bool {
        #[cfg(target_os = "windows")]
        {
            use std::ptr;
            use windows_sys::Win32::System::Threading::{
                CloseHandle, GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
                STILL_ACTIVE,
            };
            unsafe {
                let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                if handle == 0 {
                    return false;
                }
                let mut exit_code: u32 = 0;
                let res = GetExitCodeProcess(handle, &mut exit_code);
                CloseHandle(handle);
                res != 0 && exit_code == STILL_ACTIVE as u32
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let proc_dir = format!("/proc/{}", pid);
            if !Path::new(&proc_dir).exists() {
                return false;
            }

            // A terminated child may remain as a zombie briefly. It is not a
            // running application and must not keep the manager status green.
            if let Ok(stat) = fs::read_to_string(format!("{}/stat", proc_dir)) {
                if let Some(state) = stat.rsplit_once(") ").and_then(|(_, rest)| rest.chars().next()) {
                    return state != 'Z';
                }
            }
            true
        }
    }
}
