use serde_json::{json, Map, Value};
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

const SCHEMA_VERSION: &str = "0.1";
const MAX_SKILL_SCAN_DEPTH: usize = 8;

#[tauri::command]
pub fn scan_local_extensions() -> Value {
    let now = iso_now();
    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .ok()
        .map(PathBuf::from);

    let mut roots = Vec::new();
    let mut entities = Vec::new();
    let mut issues = Vec::new();

    if let Some(home) = home_dir.as_ref() {
        let skill_roots = [
            (home.join(".codex").join("skills"), "codex"),
            (home.join(".agents").join("skills"), "codex"),
            (home.join(".claude").join("skills"), "claude-code"),
        ];

        for (root, platform) in skill_roots {
            scan_skill_root(
                &root,
                platform,
                &now,
                &mut roots,
                &mut entities,
                &mut issues,
            );
        }

        register_file_root(&home.join(".codex").join("config.toml"), &mut roots);
        register_directory_root(&home.join(".claude").join("commands"), &mut roots);
        register_directory_root(&home.join(".claude").join("agents"), &mut roots);
        register_directory_root(&home.join(".claude").join("mcp-configs"), &mut roots);
    }

    let totals = calculate_totals(&entities);
    let mut machine = Map::new();
    machine.insert("platform".to_string(), json!(platform_name()));
    if let Some(path) = home_dir {
        machine.insert("homeDir".to_string(), json!(path.to_string_lossy()));
    }

    json!({
      "schemaVersion": SCHEMA_VERSION,
      "generatedAt": now,
      "machine": machine,
      "roots": roots,
      "entities": entities,
      "totals": totals,
      "issues": issues,
    })
}

fn scan_skill_root(
    root: &Path,
    platform: &str,
    discovered_at: &str,
    roots: &mut Vec<Value>,
    entities: &mut Vec<Value>,
    report_issues: &mut Vec<Value>,
) {
    if !root.exists() {
        roots.push(root_result(root, "directory", "missing", None));
        return;
    }

    if !root.is_dir() {
        roots.push(root_result(
            root,
            "directory",
            "error",
            Some("Configured skill root is not a directory."),
        ));
        return;
    }

    roots.push(root_result(root, "directory", "scanned", None));
    let mut skill_files = Vec::new();
    collect_skill_files(root, 0, &mut skill_files, report_issues);

    for skill_md in skill_files {
        entities.push(build_skill_entity(root, &skill_md, platform, discovered_at));
    }
}

fn collect_skill_files(
    dir: &Path,
    depth: usize,
    skill_files: &mut Vec<PathBuf>,
    report_issues: &mut Vec<Value>,
) {
    if depth > MAX_SKILL_SCAN_DEPTH || should_skip_path(dir) {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("path", dir),
              "severity": "low",
              "category": "path",
              "message": "SkillDesk could not read a configured directory.",
              "file": dir.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return;
        }
    };

    let mut child_dirs = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == "SKILL.md") {
            skill_files.push(path);
            return;
        }
        if path.is_dir() && !should_skip_path(&path) {
            child_dirs.push(path);
        }
    }

    for child in child_dirs {
        collect_skill_files(&child, depth + 1, skill_files, report_issues);
    }
}

fn build_skill_entity(root: &Path, skill_md: &Path, platform: &str, discovered_at: &str) -> Value {
    let skill_dir = skill_md.parent().unwrap_or(root);
    let name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "skill".to_string());
    let content = fs::read_to_string(skill_md).unwrap_or_default();
    let (title, description) = parse_skill_text(&content);
    let health = skill_health(&name, skill_md, &content, description.as_deref());
    let mut entity = Map::new();
    entity.insert(
        "id".to_string(),
        json!(format!("skill:{}:{}", platform, stable_path_id(skill_dir))),
    );
    entity.insert("kind".to_string(), json!("skill"));
    entity.insert("platform".to_string(), json!(platform));
    entity.insert("name".to_string(), json!(name));
    if let Some(value) = title {
        entity.insert("title".to_string(), json!(value));
    }
    if let Some(value) = description {
        entity.insert("description".to_string(), json!(value));
    }
    entity.insert("path".to_string(), json!(skill_dir.to_string_lossy()));
    entity.insert("source".to_string(), json!("local"));
    entity.insert("tags".to_string(), json!([]));
    entity.insert("discoveredAt".to_string(), json!(discovered_at));
    if let Some(value) = modified_iso(skill_md) {
        entity.insert("lastModified".to_string(), json!(value));
    }
    if let Some(value) = file_fingerprint(skill_md) {
        entity.insert("fingerprint".to_string(), json!(value));
    }
    entity.insert("health".to_string(), health);
    entity.insert(
        "files".to_string(),
        json!({
          "skillMd": skill_md.to_string_lossy(),
          "scripts": existing_child_paths(skill_dir, "scripts"),
          "references": existing_child_paths(skill_dir, "references"),
          "assets": existing_child_paths(skill_dir, "assets"),
        }),
    );

    Value::Object(entity)
}

fn parse_skill_text(content: &str) -> (Option<String>, Option<String>) {
    let mut title = None;
    let mut description = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if title.is_none() && trimmed.starts_with("# ") {
            title = Some(trimmed.trim_start_matches("# ").trim().to_string());
            continue;
        }
        if description.is_none()
            && !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && trimmed != "---"
        {
            description = Some(trimmed.to_string());
        }
        if title.is_some() && description.is_some() {
            break;
        }
    }

    (title, description)
}

fn skill_health(name: &str, skill_md: &Path, content: &str, description: Option<&str>) -> Value {
    let mut issues = Vec::new();

    if content.trim().is_empty() {
        issues.push(json!({
          "id": issue_id("empty-skill", skill_md),
          "severity": "medium",
          "category": "format",
          "message": "SKILL.md is empty.",
          "file": skill_md.to_string_lossy(),
          "recommendation": "Add a concise description and usage instructions.",
        }));
    }

    if description.is_none_or(|value| value.chars().count() < 24) {
        issues.push(json!({
          "id": issue_id("weak-description", skill_md),
          "severity": "low",
          "category": "metadata",
          "message": format!("Skill '{}' has a weak or missing description.", name),
          "file": skill_md.to_string_lossy(),
          "recommendation": "Add a clear first paragraph explaining when the skill should be used.",
        }));
    }

    let status = if issues.is_empty() {
        "ok"
    } else {
        "needs-review"
    };

    json!({
      "status": status,
      "issues": issues,
    })
}

fn register_directory_root(path: &Path, roots: &mut Vec<Value>) {
    if path.exists() && path.is_dir() {
        roots.push(root_result(path, "directory", "scanned", None));
    } else {
        roots.push(root_result(path, "directory", "missing", None));
    }
}

fn register_file_root(path: &Path, roots: &mut Vec<Value>) {
    if path.exists() && path.is_file() {
        roots.push(root_result(path, "file", "scanned", None));
    } else {
        roots.push(root_result(path, "file", "missing", None));
    }
}

fn root_result(path: &Path, kind: &str, status: &str, reason: Option<&str>) -> Value {
    let mut root = Map::new();
    root.insert("path".to_string(), json!(path.to_string_lossy()));
    root.insert("kind".to_string(), json!(kind));
    root.insert("status".to_string(), json!(status));
    if let Some(value) = reason {
        root.insert("reason".to_string(), json!(value));
    }
    Value::Object(root)
}

fn calculate_totals(entities: &[Value]) -> Value {
    let mut ok = 0;
    let mut needs_review = 0;
    let mut at_risk = 0;
    let mut broken = 0;
    let mut skills = 0;

    for entity in entities {
        if entity.get("kind").and_then(Value::as_str) == Some("skill") {
            skills += 1;
        }

        match entity
            .get("health")
            .and_then(|health| health.get("status"))
            .and_then(Value::as_str)
        {
            Some("ok") => ok += 1,
            Some("needs-review") => needs_review += 1,
            Some("at-risk") => at_risk += 1,
            Some("broken") => broken += 1,
            _ => {}
        }
    }

    json!({
      "entities": entities.len(),
      "skills": skills,
      "commands": 0,
      "agents": 0,
      "plugins": 0,
      "mcpServers": 0,
      "instructionFiles": 0,
      "byStatus": {
        "ok": ok,
        "needs-review": needs_review,
        "at-risk": at_risk,
        "broken": broken,
      },
    })
}

fn existing_child_paths(root: &Path, child_name: &str) -> Vec<String> {
    let path = root.join(child_name);
    if !path.exists() || !path.is_dir() {
        return Vec::new();
    }

    fs::read_dir(path)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.path().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn should_skip_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            matches!(
                name.to_ascii_lowercase().as_str(),
                "node_modules"
                    | "sessions"
                    | "history"
                    | "logs"
                    | "tmp"
                    | "temp"
                    | "backup"
                    | "backups"
                    | ".git"
            )
        })
}

fn file_fingerprint(path: &Path) -> Option<String> {
    let metadata = fs::metadata(path).ok()?;
    Some(format!(
        "size:{}:mtime:{}",
        metadata.len(),
        metadata
            .modified()
            .ok()
            .and_then(epoch_seconds)
            .unwrap_or_default()
    ))
}

fn modified_iso(path: &Path) -> Option<String> {
    fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(epoch_seconds)
        .map(iso_from_epoch_seconds)
}

fn iso_now() -> String {
    iso_from_system_time(SystemTime::now())
}

fn epoch_seconds(time: SystemTime) -> Option<u64> {
    time.duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

fn iso_from_epoch_seconds(seconds: u64) -> String {
    chrono::DateTime::from_timestamp(seconds as i64, 0)
        .map(|value| value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string())
}

fn iso_from_system_time(time: SystemTime) -> String {
    chrono::DateTime::<chrono::Utc>::from(time).to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn platform_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

fn stable_path_id(path: &Path) -> String {
    path.to_string_lossy()
        .chars()
        .map(|value| match value {
            'A'..='Z' => value.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => value,
            _ => '-',
        })
        .collect()
}

fn issue_id(prefix: &str, path: &Path) -> String {
    format!("{}:{}", prefix, stable_path_id(path))
}
