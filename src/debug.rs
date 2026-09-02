use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

struct DebugLogger {
    terminal: bool,
    file: Option<BufWriter<File>>,
}

static LOGGER: OnceLock<Mutex<DebugLogger>> = OnceLock::new();

pub fn initialize(terminal: bool, file: bool, prefix: &str) -> Result<Option<PathBuf>, String> {
    let log_path = if file {
        let executable = std::env::current_exe().map_err(|error| error.to_string())?;
        let directory = executable
            .parent()
            .ok_or_else(|| "Executable directory is unavailable".to_string())?
            .join("logs");
        fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
        let path = directory.join(format!(
            "{}-{}-{}.log",
            prefix,
            session_timestamp(),
            std::process::id()
        ));
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|error| error.to_string())?;
        Some((path, BufWriter::new(file)))
    } else {
        None
    };

    let returned_path = log_path.as_ref().map(|(path, _)| path.clone());
    LOGGER
        .set(Mutex::new(DebugLogger {
            terminal,
            file: log_path.map(|(_, file)| file),
        }))
        .map_err(|_| "Debug logger was already initialized".to_string())?;

    log("35", "DEBUG", "session started".to_string());
    Ok(returned_path)
}

pub fn file_logging_enabled() -> bool {
    LOGGER
        .get()
        .and_then(|logger| logger.lock().ok())
        .map(|logger| logger.file.is_some())
        .unwrap_or(false)
}

pub fn log(color: &str, label: &str, message: String) {
    let Some(logger) = LOGGER.get() else {
        return;
    };
    let Ok(mut logger) = logger.lock() else {
        return;
    };

    let timestamp = timestamp();
    if logger.terminal {
        eprintln!(
            "\x1b[{}m[{}][{}] {}\x1b[0m",
            color, timestamp, label, message
        );
    }
    if let Some(file) = logger.file.as_mut() {
        let _ = writeln!(file, "[{}][{}] {}", timestamp, label, message);
        let _ = file.flush();
    }
}

pub fn preview(value: &str) -> String {
    let preview: String = value.chars().take(2000).collect();
    if preview.chars().count() < value.chars().count() {
        format!("{}... ({} bytes total)", preview, value.len())
    } else {
        preview
    }
}

fn timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let day_seconds = now.as_secs() % 86_400;
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        day_seconds / 3_600,
        (day_seconds % 3_600) / 60,
        day_seconds % 60,
        now.subsec_millis()
    )
}

fn session_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let days = (now.as_secs() / 86_400) as i64;
    let (year, month, day) = civil_date(days);
    let day_seconds = now.as_secs() % 86_400;
    format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        year,
        month,
        day,
        day_seconds / 3_600,
        (day_seconds % 3_600) / 60,
        day_seconds % 60
    )
}

fn civil_date(days_since_epoch: i64) -> (i64, i64, i64) {
    let shifted = days_since_epoch + 719_468;
    let era = if shifted >= 0 {
        shifted / 146_097
    } else {
        (shifted - 146_096) / 146_097
    };
    let day_of_era = shifted - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = month_part + if month_part < 10 { 3 } else { -9 };
    (year + if month <= 2 { 1 } else { 0 }, month, day)
}

#[cfg(test)]
mod tests {
    use super::{civil_date, preview};

    #[test]
    fn preview_keeps_short_values_unchanged() {
        assert_eq!(preview("hello"), "hello");
        assert_eq!(preview("Привет"), "Привет");
    }

    #[test]
    fn preview_limits_characters_and_reports_bytes() {
        let value = "аб".repeat(1_001);
        let result = preview(&value);
        let expected_preview: String = value.chars().take(2000).collect();
        assert!(result.starts_with(&expected_preview));
        assert!(result.ends_with(&format!("... ({} bytes total)", value.len())));
    }

    #[test]
    fn civil_date_handles_epoch_and_leap_years() {
        assert_eq!(civil_date(0), (1970, 1, 1));
        assert_eq!(civil_date(18_262), (2020, 1, 1));
        assert_eq!(civil_date(18_322), (2020, 3, 1));
    }
}
