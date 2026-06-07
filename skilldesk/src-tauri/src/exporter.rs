use serde_json::Value;
use std::{env, fs, path::PathBuf};

#[tauri::command]
pub fn export_scan_report(report: Value) -> Result<String, String> {
    let export_dir = downloads_dir()
        .ok_or_else(|| "Could not resolve the Downloads directory.".to_string())?
        .join("SkillDesk");
    fs::create_dir_all(&export_dir).map_err(|error| error.to_string())?;

    let generated_at = report
        .get("generatedAt")
        .and_then(Value::as_str)
        .unwrap_or("scan-report");
    let file_name = format!("skilldesk-report-{}.json", safe_file_stamp(generated_at));
    let export_path = export_dir.join(file_name);
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;

    fs::write(&export_path, json).map_err(|error| error.to_string())?;
    Ok(export_path.to_string_lossy().to_string())
}

fn downloads_dir() -> Option<PathBuf> {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .ok()
        .map(PathBuf::from)?;
    Some(home.join("Downloads"))
}

fn safe_file_stamp(value: &str) -> String {
    let stamp: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect();
    stamp.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn creates_filesystem_safe_stamp() {
        assert_eq!(
            safe_file_stamp("2026-06-07T10:35:00Z"),
            "2026-06-07T10-35-00Z"
        );
    }

    #[test]
    fn resolves_downloads_under_home() {
        let path = downloads_dir().unwrap_or_else(|| Path::new("Downloads").to_path_buf());
        assert!(path.ends_with("Downloads"));
    }
}
