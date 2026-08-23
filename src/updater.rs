use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tar::Archive as TarArchive;
use zip::ZipArchive;

const RELEASES_URL: &str = "https://api.github.com/repos/Kyreesemm/WebFlow-Runtime/releases";
const CACHE_MAX_AGE: u64 = 12 * 60 * 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheck {
    pub status: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_url: Option<String>,
    pub asset_name: Option<String>,
    pub asset_url: Option<String>,
    pub asset_size: Option<u64>,
    pub asset_digest: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateState {
    pub status: String,
    pub percent: u8,
    pub message: String,
    pub result: Option<UpdateCheck>,
}

fn runtime_state() -> &'static Mutex<UpdateState> {
    static STATE: OnceLock<Mutex<UpdateState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(UpdateState {
        status: "idle".into(),
        percent: 0,
        message: String::new(),
        result: None,
    }))
}

pub fn state() -> UpdateState {
    runtime_state().lock().map(|state| state.clone()).unwrap_or(UpdateState {
        status: "error".into(), percent: 0, message: "Updater state unavailable".into(), result: None,
    })
}

fn set_state(status: &str, percent: u8, message: &str, result: Option<UpdateCheck>) {
    if let Ok(mut state) = runtime_state().lock() {
        state.status = status.into();
        state.percent = percent;
        state.message = message.into();
        if result.is_some() { state.result = result; }
    }
}

pub fn start_check(force: bool) {
    if runtime_state().lock().map(|state| state.status == "checking").unwrap_or(false) { return; }
    set_state("checking", 0, "Checking for updates", None);
    thread::spawn(move || {
        let result = check_for_updates(force);
        let message = match result.status.as_str() {
            "update_available" => "Update available".to_string(),
            "up_to_date" => "The latest version is installed".to_string(),
            _ => result.error.clone().unwrap_or_else(|| "Update check failed".into()),
        };
        let status = result.status.clone();
        set_state(&status, 0, &message, Some(result));
    });
}

pub fn start_update() {
    if runtime_state().lock().map(|state| state.status == "downloading" || state.status == "verifying" || state.status == "restarting").unwrap_or(true) { return; }
    let result = state().result;
    let Some(result) = result else { return; };
    thread::spawn(move || {
        let result = download_and_start_update(result, |phase, percent, message| set_state(phase, percent, message, None));
        if let Err(error) = result {
            set_state("error", 0, &error, None);
        }
    });
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    prerelease: bool,
    draft: bool,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
    digest: Option<String>,
}

fn current_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or_else(|_| Version::new(0, 0, 0))
}

fn current_version_string() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn parse_release_version(tag: &str) -> Option<Version> {
    Version::parse(tag.trim_start_matches('v')).ok()
}

fn asset_name(tag: &str) -> String {
    let platform = if cfg!(target_os = "windows") {
        "Windows"
    } else {
        "Linux"
    };
    let extension = if cfg!(target_os = "windows") { "zip" } else { "tar.gz" };
    format!("WebFlow-Runtime-{tag}-Portable-{platform}-x86-64.{extension}")
}

fn unavailable(error: impl Into<String>) -> UpdateCheck {
    UpdateCheck {
        status: "error".into(),
        current_version: current_version_string(),
        latest_version: None,
        release_url: None,
        asset_name: None,
        asset_url: None,
        asset_size: None,
        asset_digest: None,
        error: Some(error.into()),
    }
}

fn state_path() -> PathBuf {
    crate::config::Config::get_config_dir().join("update_state.json")
}

fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn cached_check(force: bool) -> Option<UpdateCheck> {
    if force {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(state_path()).ok()?).ok()?;
    let checked_at = value.get("checked_at")?.as_u64()?;
    if now_seconds().saturating_sub(checked_at) > CACHE_MAX_AGE {
        return None;
    }
    let result: UpdateCheck = serde_json::from_value(value.get("result")?.clone()).ok()?;
    (result.current_version == current_version_string()).then_some(result)
}

fn save_check(result: &UpdateCheck) {
    let state = serde_json::json!({ "checked_at": now_seconds(), "result": result });
    if let Ok(content) = serde_json::to_string_pretty(&state) {
        let _ = fs::write(state_path(), content);
    }
}

pub fn check_for_updates(force: bool) -> UpdateCheck {
    if let Some(cached) = cached_check(force) {
        return cached;
    }

    let result = check_remote_releases();
    save_check(&result);
    result
}

fn check_remote_releases() -> UpdateCheck {
    let client = match Client::builder().user_agent(format!("WebFlow-Runtime/{}", current_version_string())).build() {
        Ok(client) => client,
        Err(error) => return unavailable(format!("Failed to create HTTP client: {error}")),
    };

    let response = match client
        .get(RELEASES_URL)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
    {
        Ok(response) => response,
        Err(error) => return unavailable(format!("Failed to request GitHub Releases: {error}")),
    };
    if !response.status().is_success() {
        return unavailable(format!("GitHub Releases returned HTTP {}", response.status()));
    }

    let releases: Vec<GithubRelease> = match response.json() {
        Ok(releases) => releases,
        Err(error) => return unavailable(format!("Invalid GitHub Releases response: {error}")),
    };
    let current = current_version();
    let allow_prereleases = current.pre.is_empty() == false;

    let mut candidates: Vec<(Version, &GithubRelease)> = releases
        .iter()
        .filter(|release| !release.draft && (allow_prereleases || !release.prerelease))
        .filter_map(|release| parse_release_version(&release.tag_name).map(|version| (version, release)))
        .filter(|(version, _)| version > &current)
        .collect();
    candidates.sort_by(|left, right| left.0.cmp(&right.0));

    let Some((version, release)) = candidates.pop() else {
        return UpdateCheck {
            status: "up_to_date".into(),
            current_version: current_version_string(),
            latest_version: None,
            release_url: None,
            asset_name: None,
            asset_url: None,
            asset_size: None,
            asset_digest: None,
            error: None,
        };
    };

    let expected_name = asset_name(&release.tag_name);
    let Some(asset) = release.assets.iter().find(|asset| asset.name == expected_name) else {
        return unavailable(format!("Release {} has no compatible {} asset", version, expected_name));
    };
    if !asset.browser_download_url.starts_with("https://github.com/Kyreesemm/WebFlow-Runtime/releases/download/") {
        return unavailable("Release asset URL is not trusted");
    }
    let Some(digest) = asset.digest.as_deref().filter(|digest| digest.starts_with("sha256:")) else {
        return unavailable("Release asset has no SHA-256 digest");
    };

    UpdateCheck {
        status: "update_available".into(),
        current_version: current_version_string(),
        latest_version: Some(version.to_string()),
        release_url: Some(release.html_url.clone()),
        asset_name: Some(asset.name.clone()),
        asset_url: Some(asset.browser_download_url.clone()),
        asset_size: Some(asset.size),
        asset_digest: Some(digest.trim_start_matches("sha256:").to_string()),
        error: None,
    }
}

pub fn download_and_start_update<F>(check: UpdateCheck, mut progress: F) -> Result<(), String>
where
    F: FnMut(&str, u8, &str),
{
    if check.status != "update_available" {
        return Err("No verified update is available".into());
    }
    let url = check.asset_url.as_deref().ok_or("Update URL is missing")?;
    let digest = check.asset_digest.as_deref().ok_or("Update digest is missing")?;
    let file_name = check.asset_name.as_deref().ok_or("Update file name is missing")?;
    let runtime_dir = crate::config::Config::get_runtime_dir();
    let update_dir = runtime_dir.join("update");
    fs::create_dir_all(&update_dir).map_err(|error| error.to_string())?;
    let archive_path = update_dir.join(file_name);
    let client = Client::builder().user_agent(format!("WebFlow-Runtime/{}", current_version_string())).build().map_err(|e| e.to_string())?;
    let mut response = client.get(url).send().map_err(|error| format!("Download failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!("Download returned HTTP {}", response.status()));
    }
    let total = response.content_length().or(check.asset_size).unwrap_or(0);
    let mut output = File::create(&archive_path).map_err(|error| error.to_string())?;
    let mut hasher = Sha256::new();
    let mut downloaded = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    progress("downloading", 0, "Downloading update");
    loop {
        let read = response.read(&mut buffer).map_err(|error| error.to_string())?;
        if read == 0 { break; }
        output.write_all(&buffer[..read]).map_err(|error| error.to_string())?;
        hasher.update(&buffer[..read]);
        downloaded += read as u64;
        let percent = if total == 0 { 0 } else { ((downloaded * 75) / total).min(75) as u8 };
        progress("downloading", percent, "Downloading update");
    }
    output.sync_all().map_err(|error| error.to_string())?;
    let actual = format!("{:x}", hasher.finalize());
    progress("verifying", 85, "Verifying update integrity");
    if actual != digest {
        let _ = fs::remove_file(&archive_path);
        return Err("SHA-256 digest mismatch".into());
    }
    progress("verified", 92, "Update verified");
    let pid = std::process::id().to_string();
    Command::new(std::env::current_exe().map_err(|error| error.to_string())?)
        .arg("--update-helper")
        .arg(&archive_path)
        .arg("--update-parent-pid")
        .arg(pid)
        .spawn()
        .map_err(|error| format!("Failed to start update helper: {error}"))?;
    progress("restarting", 100, "Restarting WebFlow Runtime");
    std::process::exit(0);
}

pub fn run_update_helper(archive: &str, parent_pid: Option<u32>) -> Result<(), String> {
    if let Some(pid) = parent_pid {
        for _ in 0..100 {
            if !process_is_running(pid) { break; }
            thread::sleep(Duration::from_millis(100));
        }
        if process_is_running(pid) {
            return Err("The previous Manager process did not exit".into());
        }
    }
    let archive = Path::new(archive);
    let base_dir = crate::config::Config::get_base_dir();
    let staging = crate::config::Config::get_runtime_dir().join("update-staging");
    if staging.exists() { fs::remove_dir_all(&staging).map_err(|e| e.to_string())?; }
    fs::create_dir_all(&staging).map_err(|e| e.to_string())?;
    extract_archive(archive, &staging)?;
    let root = find_archive_root(&staging)?;
    let executable = if cfg!(target_os = "windows") { "webflow-runtime.exe" } else { "webflow-runtime" };
    if !root.join(executable).is_file() { return Err("Update archive has no runtime executable".into()); }
    copy_update_files(&root, &base_dir)?;
    let _ = fs::remove_dir_all(&staging);
    let _ = fs::remove_file(archive);
    Command::new(base_dir.join(executable)).spawn().map_err(|e| format!("Failed to restart Manager: {e}"))?;
    Ok(())
}

fn process_is_running(pid: u32) -> bool {
    #[cfg(target_os = "linux")]
    { return Path::new(&format!("/proc/{pid}")).exists(); }
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
        const STILL_ACTIVE_CODE: u32 = 259;
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if handle.is_null() { return false; }
            let mut code = 0;
            let ok = GetExitCodeProcess(handle, &mut code) != 0;
            CloseHandle(handle);
            ok && code == STILL_ACTIVE_CODE
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    { false }
}

fn extract_archive(archive: &Path, destination: &Path) -> Result<(), String> {
    if archive.extension().and_then(|e| e.to_str()) == Some("zip") {
        let file = File::open(archive).map_err(|e| e.to_string())?;
        let mut zip = ZipArchive::new(file).map_err(|e| e.to_string())?;
        for index in 0..zip.len() {
            let mut entry = zip.by_index(index).map_err(|e| e.to_string())?;
            let relative = entry.enclosed_name().ok_or("Archive contains an unsafe path")?.to_path_buf();
            let target = destination.join(relative);
            if entry.is_dir() { fs::create_dir_all(&target).map_err(|e| e.to_string())?; continue; }
            if let Some(parent) = target.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
            let mut output = File::create(target).map_err(|e| e.to_string())?;
            io::copy(&mut entry, &mut output).map_err(|e| e.to_string())?;
        }
    } else {
        let file = File::open(archive).map_err(|e| e.to_string())?;
        let decoder = GzDecoder::new(file);
        let mut tar = TarArchive::new(decoder);
        for entry in tar.entries().map_err(|e| e.to_string())? {
            let mut entry = entry.map_err(|e| e.to_string())?;
            let relative = entry.path().map_err(|e| e.to_string())?.to_path_buf();
            if relative.is_absolute() || relative.components().any(|component| component == std::path::Component::ParentDir) {
                return Err("Archive contains an unsafe path".into());
            }
            entry.unpack(destination.join(relative)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn find_archive_root(staging: &Path) -> Result<PathBuf, String> {
    let entries: Vec<_> = fs::read_dir(staging).map_err(|e| e.to_string())?.flatten().collect();
    if entries.len() == 1 && entries[0].path().is_dir() { Ok(entries[0].path()) } else { Ok(staging.to_path_buf()) }
}

fn copy_update_files(source: &Path, destination: &Path) -> Result<(), String> {
    for entry in walkdir::WalkDir::new(source).into_iter().filter_map(Result::ok) {
        let relative = entry.path().strip_prefix(source).map_err(|e| e.to_string())?;
        if relative.as_os_str().is_empty() { continue; }
        let target = destination.join(relative);
        if entry.file_type().is_dir() { fs::create_dir_all(&target).map_err(|e| e.to_string())?; continue; }
        if relative.components().next().map(|c| c.as_os_str() == "userdata").unwrap_or(false) { continue; }
        if let Some(parent) = target.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
        let temporary = target.with_extension("update-new");
        fs::copy(entry.path(), &temporary).map_err(|e| e.to_string())?;
        fs::rename(&temporary, &target).or_else(|_| { fs::remove_file(&target).map_err(|e| e.to_string())?; fs::rename(&temporary, &target).map_err(|e| e.to_string()) }).map_err(|e| e.to_string())?;
    }
    Ok(())
}
