use crate::config::{AppConfig, Config, EngineSettings};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(unix)]
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use wry::WebView;

#[derive(Debug, Deserialize)]
pub struct IpcPayload {
    pub id: u64,
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<Value>,
}

pub fn handle_ipc_message(webview_handle: Arc<Mutex<Option<WebView>>>, body: &str) {
    let payload: IpcPayload = match serde_json::from_str(body) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[IPC] Failed to parse IPC payload: {}", e);
            return;
        }
    };

    let id = payload.id;
    let cmd = payload.cmd.as_str();
    let args = payload.args;

    match cmd {
        "listApps" => {
            let app_ids = Config::list_apps();
            let mut apps = Vec::new();
            for app_id in app_ids {
                if let Some(config) = Config::load_app_config(&app_id) {
                    let icon_path = Config::get_apps_dir().join(&app_id).join("icon.png");
                    let has_icon = icon_path.exists();
                    apps.push(serde_json::json!({
                        "id": app_id,
                        "name": config.name,
                        "url": config.url,
                        "iconPath": icon_path.to_string_lossy(),
                        "hasIcon": has_icon
                    }));
                }
            }
            let json_str = serde_json::to_string(&apps).unwrap_or_else(|_| "[]".into());
            send_response(&webview_handle, id, &json_str);
        }
        "getAppConfig" => {
            if let Some(app_id) = args.get(0).and_then(|v| v.as_str()) {
                if let Some(config) = Config::load_app_config(app_id) {
                    let json_str = serde_json::to_string(&config).unwrap_or_else(|_| "{}".into());
                    send_response(&webview_handle, id, &json_str);
                    return;
                }
            }
            send_response(&webview_handle, id, "null");
        }
        "createApp" | "updateApp" => {
            if let (Some(app_id), Some(config_val)) = (
                args.get(0).and_then(|v| v.as_str()),
                args.get(1),
            ) {
                let config: Result<AppConfig, _> = if config_val.is_string() {
                    serde_json::from_str(config_val.as_str().unwrap_or(""))
                } else {
                    serde_json::from_value(config_val.clone())
                };

                if let Ok(cfg) = config {
                    let _ = Config::save_app_config(app_id, &cfg);
                    send_response(&webview_handle, id, "true");
                    return;
                }
            }
            send_response(&webview_handle, id, "false");
        }
        "createAppWithIcon" | "updateAppWithIcon" => {
            if let (Some(app_id), Some(config_val), Some(icon_b64)) = (
                args.get(0).and_then(|v| v.as_str()),
                args.get(1),
                args.get(2).and_then(|v| v.as_str()),
            ) {
                let mut config: Result<AppConfig, _> = if config_val.is_string() {
                    serde_json::from_str(config_val.as_str().unwrap_or(""))
                } else {
                    serde_json::from_value(config_val.clone())
                };

                if let Ok(ref mut cfg) = config {
                    if let Some(clean_b64) = icon_b64.split(',').last() {
                        use base64::Engine;
                        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(clean_b64) {
                            let icon_path = Config::get_apps_dir().join(app_id).join("icon.png");
                            let _ = fs::create_dir_all(Config::get_apps_dir().join(app_id));
                            let _ = fs::write(icon_path, bytes);
                            cfg.icon = Some("icon.png".to_string());
                        }
                    }
                    let _ = Config::save_app_config(app_id, cfg);
                    send_response(&webview_handle, id, "true");
                    return;
                }
            }
            send_response(&webview_handle, id, "false");
        }
        "deleteApp" => {
            if let Some(app_id) = args.get(0).and_then(|v| v.as_str()) {
                let _ = Config::delete_app(app_id);
                send_response(&webview_handle, id, "true");
            } else {
                send_response(&webview_handle, id, "false");
            }
        }
        "runApp" => {
            if let Some(app_id) = args.get(0).and_then(|v| v.as_str()) {
                if let Ok(exe) = std::env::current_exe() {
                    let _ = Command::new(exe)
                        .arg("--app")
                        .arg(app_id)
                        .spawn();
                    send_response(&webview_handle, id, "true");
                    return;
                }
            }
            send_response(&webview_handle, id, "false");
        }
        "getRunningApps" => {
            let running = Config::get_running_apps();
            let json_str = serde_json::to_string(&running).unwrap_or_else(|_| "[]".into());
            send_response(&webview_handle, id, &json_str);
        }
        "listTemplates" => {
            let templates = list_templates_internal();
            let json_str = serde_json::to_string(&templates).unwrap_or_else(|_| "[]".into());
            send_response(&webview_handle, id, &json_str);
        }
        "createFromTemplate" => {
            if let Some(template_id) = args.get(0).and_then(|v| v.as_str()) {
                if let Ok(app_id) = create_from_template_internal(template_id) {
                    send_response(&webview_handle, id, &serde_json::to_string(&app_id).unwrap_or_default());
                    return;
                }
            }
            send_response(&webview_handle, id, "null");
        }
        "getEngineSettings" => {
            let settings = Config::load_engine_settings();
            let mut val = serde_json::to_value(&settings).unwrap_or_default();
            if let Some(map) = val.as_object_mut() {
                map.insert("current_userdata_path".into(), Value::String(Config::get_userdata_base().to_string_lossy().to_string()));
                map.insert("current_apps_path".into(), Value::String(Config::get_apps_dir().to_string_lossy().to_string()));
                map.insert("current_config_path".into(), Value::String(Config::get_config_dir().to_string_lossy().to_string()));
                map.insert("current_runtime_path".into(), Value::String(Config::get_runtime_dir().to_string_lossy().to_string()));
                map.insert("current_shared_storage_path".into(), Value::String(Config::get_shared_storage_dir().to_string_lossy().to_string()));
            }
            send_response(&webview_handle, id, &serde_json::to_string(&val).unwrap_or_else(|_| "{}".into()));
        }
        "updateEngineSettings" => {
            if let Some(val) = args.get(0) {
                let settings: Result<EngineSettings, _> = if val.is_string() {
                    serde_json::from_str(val.as_str().unwrap_or(""))
                } else {
                    serde_json::from_value(val.clone())
                };

                if let Ok(st) = settings {
                    let _ = Config::save_engine_settings(&st);
                    send_response(&webview_handle, id, "true");
                    return;
                }
            }
            send_response(&webview_handle, id, "false");
        }
        "listUserAgents" => {
            let mut uas = vec![
                serde_json::json!({"id": "default", "name": "Default", "string": "WebKitGTK default", "custom": false}),
                serde_json::json!({"id": "linux", "name": "Linux", "string": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36", "custom": false}),
                serde_json::json!({"id": "windows", "name": "Windows", "string": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36", "custom": false}),
                serde_json::json!({"id": "chrome-linux", "name": "Google Chrome (Linux)", "string": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36", "custom": false}),
                serde_json::json!({"id": "chrome-windows", "name": "Google Chrome (Windows)", "string": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/151.0.0.0 Safari/537.36", "custom": false}),
                serde_json::json!({"id": "iphone", "name": "iPhone", "string": "Mozilla/5.0 (iPhone; CPU iPhone OS 18_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Mobile/15E148 Safari/604.1", "custom": false}),
                serde_json::json!({"id": "android", "name": "Android", "string": "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.6778.200 Mobile Safari/537.36", "custom": false}),
            ];
            let custom = load_user_agents_internal();
            if let Some(obj) = custom.as_object() {
                for (ua_id, ua_data) in obj {
                    if let Some(obj) = ua_data.as_object() {
                        uas.push(serde_json::json!({
                            "id": ua_id,
                            "name": obj.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                            "string": obj.get("string").and_then(|v| v.as_str()).unwrap_or(""),
                            "custom": true
                        }));
                    }
                }
            }
            send_response(&webview_handle, id, &serde_json::to_string(&uas).unwrap_or_else(|_| "[]".into()));
        }
        "addUserAgent" => {
            if let (Some(name), Some(ua_str)) = (
                args.get(0).and_then(|v| v.as_str()),
                args.get(1).and_then(|v| v.as_str()),
            ) {
                add_user_agent_internal(name, ua_str);
                send_response(&webview_handle, id, "true");
            } else {
                send_response(&webview_handle, id, "false");
            }
        }
        "deleteUserAgent" => {
            if let Some(ua_id) = args.get(0).and_then(|v| v.as_str()) {
                delete_user_agent_internal(ua_id);
                send_response(&webview_handle, id, "true");
            } else {
                send_response(&webview_handle, id, "false");
            }
        }
        "getWindowState" => {
            let path = Config::get_config_dir().join("window_state.json");
            let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".into());
            send_response(&webview_handle, id, &content);
        }
        "saveWindowState" => {
            if let Some(val) = args.get(0) {
                let incoming = if val.is_string() {
                    serde_json::from_str::<Value>(val.as_str().unwrap_or("{}"))
                        .unwrap_or_else(|_| serde_json::json!({}))
                } else {
                    val.clone()
                };
                let path = Config::get_config_dir().join("window_state.json");
                let mut state = fs::read_to_string(&path)
                    .ok()
                    .and_then(|content| serde_json::from_str::<Value>(&content).ok())
                    .unwrap_or_else(|| serde_json::json!({}));

                if !state.is_object() {
                    state = serde_json::json!({});
                }
                if let (Some(state), Some(incoming)) = (state.as_object_mut(), incoming.as_object()) {
                    for (key, value) in incoming {
                        state.insert(key.clone(), value.clone());
                    }
                }

                if let Ok(json_str) = serde_json::to_string_pretty(&state) {
                    let _ = fs::write(path, json_str);
                }
            }
            send_response(&webview_handle, id, "true");
        }
        "selectFolder" => {
            let (tx, rx) = mpsc::channel();
            std::thread::spawn(move || {
                let res = rfd::FileDialog::new().pick_folder();
                let path_str = res.map(|p| p.to_string_lossy().to_string());
                let _ = tx.send(path_str);
            });

            if let Ok(Some(folder)) = rx.recv() {
                send_response(&webview_handle, id, &folder);
            } else {
                send_response(&webview_handle, id, "");
            }
        }
        "openFolder" => {
            if let Some(ftype) = args.get(0).and_then(|v| v.as_str()) {
                let path = match ftype {
                    "apps" => Config::get_apps_dir(),
                    "config" => Config::get_config_dir(),
                    "runtime" => Config::get_runtime_dir(),
                    "shared_storage" => Config::get_shared_storage_dir(),
                    _ => Config::get_userdata_base(),
                };
                let _ = open::that(path);
                send_response(&webview_handle, id, "true");
            } else {
                send_response(&webview_handle, id, "false");
            }
        }
        "clearAppCache" | "clearAppData" => {
            if let Some(app_id) = args.get(0).and_then(|v| v.as_str()) {
                let storage = Config::get_apps_dir().join(app_id).join("storage");
                if cmd == "clearAppCache" {
                    clear_webview_cache(&storage);
                } else {
                    clear_webview_cookies(&storage);
                }
                send_response(&webview_handle, id, "true");
            } else {
                send_response(&webview_handle, id, "false");
            }
        }
        "clearAllCache" | "clearAllData" => {
            let apps_dir = Config::get_apps_dir();
            if let Ok(entries) = fs::read_dir(apps_dir) {
                for entry in entries.flatten() {
                    let storage = entry.path().join("storage");
                    if cmd == "clearAllCache" {
                        clear_webview_cache(&storage);
                    } else {
                        clear_webview_cookies(&storage);
                    }
                }
            }
            let shared = Config::get_shared_storage_dir();
            if cmd == "clearAllCache" {
                clear_webview_cache(&shared);
            } else {
                clear_webview_cookies(&shared);
            }
            send_response(&webview_handle, id, "true");
        }
        "getTotalCacheSize" => {
            let shared = Config::get_shared_storage_dir();
            let total = directory_size(&shared.join("WebKitCache"))
                + directory_size(&shared.join("CacheStorage"));
            let formatted = format_size(total);
            send_response(&webview_handle, id, &formatted);
        }
        "getTotalDataSize" => {
            // Data excludes the two cache trees shown by getTotalCacheSize.
            let shared = Config::get_shared_storage_dir();
            let total = directory_size_excluding(&shared, &["WebKitCache", "CacheStorage"]);
            let formatted = format_size(total);
            send_response(&webview_handle, id, &formatted);
        }
        "getAppStorageSizes" => {
            let mut sizes = Vec::new();
            for app_id in Config::list_apps() {
                if let Some(config) = Config::load_app_config(&app_id) {
                    if config.isolated_storage {
                        sizes.push(serde_json::json!({
                            "id": app_id,
                            "name": config.name,
                            "size": format_size(directory_size(
                                &Config::get_apps_dir().join(&app_id).join("storage")
                            ))
                        }));
                    }
                }
            }
            send_response(&webview_handle, id, &serde_json::to_string(&sizes).unwrap_or_else(|_| "[]".into()));
        }
        "listCookieBrowsers" => {
            send_response(&webview_handle, id, "[]");
        }
        "importBrowserCookies" => {
            send_response(&webview_handle, id, "false");
        }
        _ => {
            eprintln!("[IPC] Unknown command: {}", cmd);
            send_response(&webview_handle, id, "null");
        }
    }
}

fn send_response(webview_handle: &Arc<Mutex<Option<WebView>>>, id: u64, json_data: &str) {
    if let Ok(guard) = webview_handle.lock() {
        if let Some(ref webview) = *guard {
            // The legacy Qt WebChannel API returns JSON text. The manager UI
            // parses every response, including arrays and objects, as JSON.
            // Encode the complete payload as a JS string before invoking the
            // callback so Rust and the old frontend keep the same contract.
            let js_data = serde_json::to_string(json_data).unwrap_or_else(|_| "\"null\"".into());
            let script = format!(
                "if (window.__WEBFLOW_IPC_CALLBACK__) window.__WEBFLOW_IPC_CALLBACK__({}, {});",
                id, js_data
            );
            let _ = webview.evaluate_script(&script);
        }
    }
}

fn format_size(bytes: u64) -> String {
    // Use decimal units to match the values shown by Nautilus.
    if bytes < 1_000 {
        format!("{} B", bytes)
    } else if bytes < 1_000_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else if bytes < 1_000_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    }
}

fn directory_size(path: &std::path::Path) -> u64 {
    #[cfg(unix)]
    let mut seen_files = HashSet::new();
    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;

            // WebKitGTK commonly hard-links cache blobs from Blobs/ into
            // Records/.../Resource/. Count each inode once, like `du` and
            // file managers do, instead of counting the same bytes twice.
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if !seen_files.insert((metadata.dev(), metadata.ino())) {
                    return None;
                }
            }

            Some(metadata.len())
        })
        .sum()
}

fn directory_size_excluding(path: &Path, excluded_roots: &[&str]) -> u64 {
    #[cfg(unix)]
    let mut seen_files = HashSet::new();
    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|entry| {
            entry
                .path()
                .strip_prefix(path)
                .ok()
                .and_then(|relative| relative.components().next())
                .and_then(|component| component.as_os_str().to_str())
                .map(|root| !excluded_roots.contains(&root))
                .unwrap_or(true)
        })
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if !seen_files.insert((metadata.dev(), metadata.ino())) {
                    return None;
                }
            }
            Some(metadata.len())
        })
        .sum()
}

fn clear_webview_cache(storage: &Path) {
    for directory in ["WebKitCache", "CacheStorage"] {
        let _ = fs::remove_dir_all(storage.join(directory));
    }
}

fn clear_webview_cookies(storage: &Path) {
    if let Ok(entries) = fs::read_dir(storage) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name == "cookies" || name.starts_with("cookies-"))
                .unwrap_or(false)
            {
                let _ = fs::remove_file(path);
            }
        }
    }
}

#[derive(Serialize)]
struct TemplateInfo {
    id: String,
    name: String,
    description: String,
    url: String,
}

fn list_templates_internal() -> Vec<TemplateInfo> {
    let mut list = Vec::new();

    list.push(TemplateInfo {
        id: "claude".into(),
        name: "Claude AI".into(),
        description: "Anthropic Claude Assistant".into(),
        url: "https://claude.ai".into(),
    });
    list.push(TemplateInfo {
        id: "chatgpt".into(),
        name: "ChatGPT".into(),
        description: "OpenAI ChatGPT".into(),
        url: "https://chatgpt.com".into(),
    });
    list.push(TemplateInfo {
        id: "deepseek".into(),
        name: "DeepSeek".into(),
        description: "DeepSeek AI Chat".into(),
        url: "https://chat.deepseek.com".into(),
    });
    list.push(TemplateInfo {
        id: "youtube".into(),
        name: "YouTube".into(),
        description: "YouTube Video Streaming".into(),
        url: "https://youtube.com".into(),
    });

    let tmpl_dir = Config::get_base_dir().join("templates");
    if tmpl_dir.exists() {
        if let Ok(entries) = fs::read_dir(tmpl_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && p.join("config.json").exists() {
                    if let Ok(content) = fs::read_to_string(p.join("config.json")) {
                        if let Ok(cfg) = serde_json::from_str::<Value>(&content) {
                            let name = cfg.get("name").and_then(|v| v.as_str()).unwrap_or("Template").to_string();
                            let desc = cfg.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let url = cfg.get("url").and_then(|v| v.as_str()).unwrap_or("https://example.com").to_string();
                            if let Some(id) = p.file_name().and_then(|s| s.to_str()) {
                                list.push(TemplateInfo {
                                    id: id.to_string(),
                                    name,
                                    description: desc,
                                    url,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    list
}

fn create_from_template_internal(template_id: &str) -> Result<String, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let app_id = format!("{}-{}", template_id, now);

    let (name, url) = match template_id {
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

    Config::save_app_config(&app_id, &config)?;
    Ok(app_id)
}

fn load_user_agents_internal() -> Value {
    let path = Config::get_config_dir().join("user_agents.json");
    if !path.exists() {
        return serde_json::json!({});
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_else(|| serde_json::json!({}))
}

fn add_user_agent_internal(name: &str, string: &str) {
    let mut uas = load_user_agents_internal();
    let ua_id = name.to_lowercase().replace(' ', "-");
    if let Some(obj) = uas.as_object_mut() {
        obj.insert(
            ua_id,
            serde_json::json!({
                "name": name,
                "string": string
            }),
        );
    }
    let path = Config::get_config_dir().join("user_agents.json");
    let _ = fs::write(path, serde_json::to_string_pretty(&uas).unwrap_or_default());
}

fn delete_user_agent_internal(ua_id: &str) {
    let mut uas = load_user_agents_internal();
    if let Some(obj) = uas.as_object_mut() {
        obj.remove(ua_id);
    }
    let path = Config::get_config_dir().join("user_agents.json");
    let _ = fs::write(path, serde_json::to_string_pretty(&uas).unwrap_or_default());
}
