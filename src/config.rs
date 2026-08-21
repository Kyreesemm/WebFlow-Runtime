use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::ptr::{null, null_mut};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

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
    #[serde(default = "default_true")]
    pub isolated_storage: bool,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default)]
    pub start_minimized: bool,
    #[serde(default)]
    pub minimize_to_tray: bool,
    #[serde(default)]
    pub app_tray_icons: bool,
    #[serde(default)]
    pub tray_apps_menu: bool,
    #[serde(default)]
    pub google_oauth_fallback: bool,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            userdata_path: None,
            isolated_storage: true,
            autostart: false,
            start_minimized: false,
            minimize_to_tray: false,
            app_tray_icons: false,
            tray_apps_menu: false,
            google_oauth_fallback: false,
        }
    }
}

pub struct Config;

impl Config {
    pub fn is_gnome_session() -> bool {
        #[cfg(target_os = "linux")]
        {
            return ["XDG_CURRENT_DESKTOP", "XDG_SESSION_DESKTOP", "DESKTOP_SESSION"]
                .iter()
                .filter_map(|name| std::env::var(name).ok())
                .flat_map(|value| value.split(':').map(str::to_owned).collect::<Vec<_>>())
                .any(|value| value.to_ascii_lowercase().contains("gnome"));
        }

        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

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
        let base_dir = Self::get_base_dir();
        let settings_files = [
            base_dir.join("engine_settings.json"),
            base_dir.join("userdata").join("config").join("engine_settings.json"),
        ];

        for settings_file in settings_files {
            if let Ok(content) = fs::read_to_string(settings_file) {
                if let Ok(st) = serde_json::from_str::<EngineSettings>(&content) {
                    if let Some(path) = st.userdata_path {
                        let path = PathBuf::from(path);
                        if path.exists() || fs::create_dir_all(&path).is_ok() {
                            return path;
                        }
                    }
                }
            }
        }

        let base = base_dir.join("userdata");
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
        let mut settings: EngineSettings = fs::read_to_string(path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default();
        if Self::is_gnome_session() {
            settings.minimize_to_tray = false;
        }
        settings
    }

    pub fn save_engine_settings(settings: &EngineSettings) -> Result<(), String> {
        let path = Self::get_config_dir().join("engine_settings.json");
        Self::save_engine_settings_at(&path, settings)?;
        Self::sync_autostart(settings)
    }

    fn sync_autostart(settings: &EngineSettings) -> Result<(), String> {
        #[cfg(target_os = "linux")]
        {
            let config_dir = std::env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
                .ok_or_else(|| "Unable to determine the user config directory".to_string())?;
            let autostart_dir = config_dir.join("autostart");
            let entry_path = autostart_dir.join("webflow-runtime-manager.desktop");

            if !settings.autostart {
                if let Err(error) = fs::remove_file(&entry_path) {
                    if error.kind() != std::io::ErrorKind::NotFound {
                        return Err(error.to_string());
                    }
                }
                Ok(())
            } else {
                fs::create_dir_all(&autostart_dir).map_err(|error| error.to_string())?;
                let executable = std::env::current_exe().map_err(|error| error.to_string())?;
                let executable = executable.to_string_lossy().replace('"', "\\\"");
                let icon_path = Self::get_base_dir()
                    .join("materials")
                    .join("default")
                    .join("webflow_runtime_icon.png")
                    .to_string_lossy()
                    .replace('\\', "\\\\")
                    .replace(' ', "\\s");
                let content = format!(
                    "[Desktop Entry]\nType=Application\nName=WebFlow Runtime Manager\nExec=\"{}\" --autostart\nIcon={}\nTerminal=false\nStartupNotify=true\nX-GNOME-Autostart-enabled=true\n",
                    executable, icon_path
                );
                fs::write(entry_path, content).map_err(|error| error.to_string())
            }
        }

        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::System::Registry::{
                RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW, RegSetValueExW,
                HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ,
            };

            const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
            const VALUE_NAME: &str = "WebFlowRuntimeManager";
            let run_key: Vec<u16> = std::ffi::OsStr::new(RUN_KEY)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            let value_name: Vec<u16> = std::ffi::OsStr::new(VALUE_NAME)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            if !settings.autostart {
                let mut key: HKEY = null_mut();
                let result = unsafe {
                    RegOpenKeyExW(HKEY_CURRENT_USER, run_key.as_ptr(), 0, KEY_SET_VALUE, &mut key)
                };
                if result == 2 {
                    return Ok(());
                }
                if result != 0 {
                    return Err(format!("Failed to open Windows autostart registry key: {result}"));
                }
                let result = unsafe { RegDeleteValueW(key, value_name.as_ptr()) };
                unsafe { RegCloseKey(key); }
                if result != 0 && result != 2 {
                    return Err(format!("Failed to remove Windows autostart entry: {result}"));
                }
                return Ok(());
            }

            let executable = std::env::current_exe().map_err(|error| error.to_string())?;
            let command = format!("\"{}\" --autostart", executable.to_string_lossy().replace('"', "\\\""));
            let command: Vec<u16> = std::ffi::OsStr::new(&command)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            let mut key: HKEY = null_mut();
            let result = unsafe {
                RegCreateKeyExW(
                    HKEY_CURRENT_USER,
                    run_key.as_ptr(),
                    0,
                    null(),
                    0,
                    KEY_SET_VALUE,
                    null(),
                    &mut key,
                    null_mut(),
                )
            };
            if result != 0 {
                return Err(format!("Failed to open Windows autostart registry key: {result}"));
            }
            let result = unsafe {
                RegSetValueExW(
                    key,
                    value_name.as_ptr(),
                    0,
                    REG_SZ,
                    command.as_ptr().cast(),
                    (command.len() * std::mem::size_of::<u16>()) as u32,
                )
            };
            unsafe { RegCloseKey(key); }
            if result != 0 {
                return Err(format!("Failed to save Windows autostart entry: {result}"));
            }
            Ok(())
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            let _ = settings;
            Ok(())
        }
    }

    fn save_engine_settings_at(path: &Path, settings: &EngineSettings) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    /// Changes the userdata root, optionally copying its contents and/or
    /// removing the old root after the new settings have been written.
    pub fn change_userdata_path(
        new_path: &str,
        transfer_data: bool,
        delete_old: bool,
    ) -> Result<(), String> {
        let old_path = Self::get_userdata_base();
        let new_path = PathBuf::from(new_path);

        if new_path.as_os_str().is_empty() {
            return Err("Путь к пользовательским данным не может быть пустым".into());
        }
        if new_path.exists() && !new_path.is_dir() {
            return Err("Выбранный путь не является папкой".into());
        }
        fs::create_dir_all(&new_path).map_err(|e| format!("Не удалось создать новую папку: {}", e))?;

        let old_canonical = fs::canonicalize(&old_path).map_err(|e| e.to_string())?;
        let new_canonical = fs::canonicalize(&new_path).map_err(|e| e.to_string())?;
        if old_canonical == new_canonical {
            return Ok(());
        }
        if new_canonical.starts_with(&old_canonical) {
            return Err("Новая папка не может находиться внутри старой папки".into());
        }
        if delete_old && (old_canonical.parent().is_none() || old_canonical == Self::get_base_dir()) {
            return Err("Нельзя удалить системную папку пользовательских данных".into());
        }

        if transfer_data && old_path.exists() {
            for entry in walkdir::WalkDir::new(&old_path).into_iter().flatten() {
                let relative = entry.path().strip_prefix(&old_path).map_err(|e| e.to_string())?;
                if relative.as_os_str().is_empty() {
                    continue;
                }
                let target = new_path.join(relative);
                if entry.file_type().is_dir() {
                    fs::create_dir_all(&target).map_err(|e| e.to_string())?;
                } else if entry.file_type().is_file() {
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                    }
                    fs::copy(entry.path(), &target).map_err(|e| {
                        format!("Не удалось перенести {}: {}", entry.path().display(), e)
                    })?;
                }
            }
        }

        let mut settings = Self::load_engine_settings();
        settings.userdata_path = Some(new_path.to_string_lossy().to_string());
        Self::save_engine_settings_at(&new_path.join("config").join("engine_settings.json"), &settings)?;

        if delete_old {
            fs::remove_dir_all(&old_path)
                .map_err(|e| format!("Новая папка сохранена, но старую удалить не удалось: {}", e))?;
        }
        Ok(())
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
            use std::ptr::null_mut;
            use windows_sys::Win32::Foundation::CloseHandle;
            use windows_sys::Win32::System::Threading::{
                GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
            };
            unsafe {
                let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                if handle == null_mut() {
                    return false;
                }
                let mut exit_code: u32 = 0;
                let res = GetExitCodeProcess(handle, &mut exit_code);
                CloseHandle(handle);
                res != 0 && exit_code == 259
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
