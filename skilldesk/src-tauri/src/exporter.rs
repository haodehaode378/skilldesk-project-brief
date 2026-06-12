use serde_json::Value;
use std::{env, fs, path::PathBuf};

#[tauri::command]
pub fn export_scan_report(report: Value) -> Result<String, String> {
    let export_dir = downloads_dir()
        .ok_or_else(|| "Could not resolve the Downloads directory.".to_string())?
        .join("SkillDesk");
    fs::create_dir_all(&export_dir).map_err(|error| error.to_string())?;

    let generated_at = report_generated_at(&report).unwrap_or("scan-report");
    let file_name = format!("skilldesk-report-{}.json", safe_file_stamp(generated_at));
    let export_path = export_dir.join(file_name);
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;

    fs::write(&export_path, json).map_err(|error| error.to_string())?;
    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn export_markdown_report(markdown: String, generated_at: String) -> Result<String, String> {
    let export_dir = downloads_dir()
        .ok_or_else(|| "Could not resolve the Downloads directory.".to_string())?
        .join("SkillDesk");
    fs::create_dir_all(&export_dir).map_err(|error| error.to_string())?;

    let file_name = format!("skilldesk-report-{}.md", safe_file_stamp(&generated_at));
    let export_path = export_dir.join(file_name);

    fs::write(&export_path, markdown).map_err(|error| error.to_string())?;
    Ok(export_path.to_string_lossy().to_string())
}

fn report_generated_at(report: &Value) -> Option<&str> {
    report
        .get("scanReport")
        .and_then(|value| value.get("generatedAt"))
        .and_then(Value::as_str)
        .or_else(|| report.get("generatedAt").and_then(Value::as_str))
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
    fn resolves_wrapped_report_generated_at() {
        let report = serde_json::json!({
            "scanReport": {
                "generatedAt": "2026-06-07T10:35:00Z"
            }
        });

        assert_eq!(report_generated_at(&report), Some("2026-06-07T10:35:00Z"));
    }

    #[test]
    fn resolves_legacy_report_generated_at() {
        let report = serde_json::json!({
            "generatedAt": "2026-06-07T10:40:00Z"
        });

        assert_eq!(report_generated_at(&report), Some("2026-06-07T10:40:00Z"));
    }

    #[test]
    fn creates_markdown_file_name_stamp() {
        assert_eq!(
            safe_file_stamp("2026-06-07T12:00:00.000Z"),
            "2026-06-07T12-00-00-000Z"
        );
    }

    #[test]
    fn resolves_downloads_under_home() {
        let path = downloads_dir().unwrap_or_else(|| Path::new("Downloads").to_path_buf());
        assert!(path.ends_with("Downloads"));
    }
}
