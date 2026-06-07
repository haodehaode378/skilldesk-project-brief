use serde_json::{json, Map, Value};
use std::{fs, path::Path, time::SystemTime};

pub(crate) fn root_result(path: &Path, kind: &str, status: &str, reason: Option<&str>) -> Value {
    let mut root = Map::new();
    root.insert("path".to_string(), json!(path.to_string_lossy()));
    root.insert("kind".to_string(), json!(kind));
    root.insert("status".to_string(), json!(status));
    if let Some(value) = reason {
        root.insert("reason".to_string(), json!(value));
    }
    Value::Object(root)
}

pub(crate) fn calculate_totals(entities: &[Value]) -> Value {
    let mut ok = 0;
    let mut needs_review = 0;
    let mut at_risk = 0;
    let mut broken = 0;
    let mut skills = 0;
    let mut commands = 0;
    let mut agents = 0;
    let mut mcp_servers = 0;
    let mut plugins = 0;
    let mut instruction_files = 0;

    for entity in entities {
        match entity.get("kind").and_then(Value::as_str) {
            Some("skill") => skills += 1,
            Some("command") => commands += 1,
            Some("agent") => agents += 1,
            Some("plugin") => plugins += 1,
            Some("mcp-server") => mcp_servers += 1,
            Some("instruction-file") => instruction_files += 1,
            _ => {}
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
      "commands": commands,
      "agents": agents,
      "plugins": plugins,
      "mcpServers": mcp_servers,
      "instructionFiles": instruction_files,
      "byStatus": {
        "ok": ok,
        "needs-review": needs_review,
        "at-risk": at_risk,
        "broken": broken,
      },
    })
}

pub(crate) fn manifest_count(manifest: &Value, key: &str) -> Option<usize> {
    let value = manifest.get(key)?;
    if let Some(items) = value.as_array() {
        return Some(items.len());
    }
    if let Some(items) = value.as_object() {
        return Some(items.len());
    }
    None
}

pub(crate) fn child_count(root: &Path, child_name: &str) -> usize {
    let path = root.join(child_name);
    if !path.exists() || !path.is_dir() {
        return 0;
    }

    fs::read_dir(path)
        .map(|entries| entries.flatten().count())
        .unwrap_or_default()
}

pub(crate) fn path_contains(path: &Path, needle: &str) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|value| value.eq_ignore_ascii_case(needle))
    })
}

pub(crate) fn mcp_transport(
    config: &Value,
    command: Option<&str>,
    url: Option<&str>,
) -> &'static str {
    if let Some(transport) = config.get("transport").and_then(Value::as_str) {
        return match transport {
            "stdio" => "stdio",
            "sse" => "sse",
            "streamable-http" => "streamable-http",
            "http" => "http",
            _ => "unknown",
        };
    }

    if command.is_some() {
        "stdio"
    } else if url.is_some() {
        "http"
    } else {
        "unknown"
    }
}

pub(crate) fn codex_mcp_tables(parsed: &toml::Value) -> Vec<(String, toml::Value)> {
    for key in ["mcp_servers", "mcpServers"] {
        if let Some(table) = parsed.get(key).and_then(toml::Value::as_table) {
            return table
                .iter()
                .map(|(name, config)| (name.to_string(), config.clone()))
                .collect();
        }
    }

    Vec::new()
}

pub(crate) fn codex_mcp_transport(
    config: &toml::Value,
    command: Option<&str>,
    url: Option<&str>,
) -> &'static str {
    if let Some(transport) = toml_string(config, "transport") {
        return match transport.as_str() {
            "stdio" => "stdio",
            "sse" => "sse",
            "streamable-http" => "streamable-http",
            "http" => "http",
            _ => "unknown",
        };
    }

    if command.is_some() {
        "stdio"
    } else if url.is_some() {
        "http"
    } else {
        "unknown"
    }
}

pub(crate) fn toml_string(config: &toml::Value, key: &str) -> Option<String> {
    config
        .get(key)
        .and_then(toml::Value::as_str)
        .map(ToString::to_string)
}

pub(crate) fn url_host(url: &str) -> Option<String> {
    let without_scheme = url.split_once("://").map(|(_, rest)| rest).unwrap_or(url);
    without_scheme
        .split('/')
        .next()
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

pub(crate) fn instruction_file_type(name: &str) -> &str {
    match name {
        "AGENTS.md" => "AGENTS.md",
        "CLAUDE.md" => "CLAUDE.md",
        ".mcp.json" => ".mcp.json",
        _ => "other",
    }
}

pub(crate) fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

pub(crate) fn file_stem(path: &Path, fallback: &str) -> String {
    path.file_stem()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| fallback.to_string())
}

pub(crate) fn namespace_for(root: &Path, file: &Path) -> Option<String> {
    let parent = file.parent()?;
    let relative = parent.strip_prefix(root).ok()?;
    let namespace = relative.to_string_lossy().replace('\\', "/");
    if namespace.is_empty() {
        None
    } else {
        Some(namespace)
    }
}

pub(crate) fn parse_declared_tools(content: &str) -> Vec<String> {
    parse_declared_field(content, "tools")
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn parse_declared_field(content: &str, key: &str) -> Option<String> {
    let field_prefix = format!("{}:", key);
    for line in content.lines().take(40) {
        let trimmed = line.trim();
        if trimmed.starts_with(&field_prefix) {
            let value = trimmed.trim_start_matches(&field_prefix).trim();
            if !value.is_empty() {
                return Some(value.trim_matches('"').trim_matches('\'').to_string());
            }
        }
    }
    None
}

pub(crate) fn existing_child_paths(root: &Path, child_name: &str) -> Vec<String> {
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

pub(crate) fn should_skip_path(path: &Path) -> bool {
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

pub(crate) fn file_fingerprint(path: &Path) -> Option<String> {
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

pub(crate) fn modified_iso(path: &Path) -> Option<String> {
    fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(epoch_seconds)
        .map(iso_from_epoch_seconds)
}

pub(crate) fn iso_now() -> String {
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

pub(crate) fn platform_name() -> &'static str {
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

pub(crate) fn stable_path_id(path: &Path) -> String {
    path.to_string_lossy()
        .chars()
        .map(|value| match value {
            'A'..='Z' => value.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => value,
            _ => '-',
        })
        .collect()
}

pub(crate) fn issue_id(prefix: &str, path: &Path) -> String {
    format!("{}:{}", prefix, stable_path_id(path))
}
