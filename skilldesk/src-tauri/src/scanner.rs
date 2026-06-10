mod common;
mod git;
mod health;

use common::*;
use git::*;
use health::*;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::{
    collections::HashSet,
    env, fs,
    net::{TcpStream, ToSocketAddrs},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const SCHEMA_VERSION: &str = "0.1";
const MAX_SKILL_SCAN_DEPTH: usize = 8;
const MAX_MARKDOWN_SCAN_DEPTH: usize = 8;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanOptions {
    include_plugin_caches: bool,
    mcp_probe_policy: Option<String>,
    scan_roots: Option<Vec<String>>,
}

#[tauri::command]
pub fn scan_local_extensions(options: Option<ScanOptions>) -> Value {
    let options = options.unwrap_or_default();
    let mcp_probe_policy = options.mcp_probe_policy.as_deref().unwrap_or("disabled");
    let now = iso_now();
    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .ok()
        .map(PathBuf::from);

    let mut roots = Vec::new();
    let mut entities = Vec::new();
    let mut issues = Vec::new();

    if let Some(home) = home_dir.as_ref() {
        let configured_roots = configured_scan_roots(home, options.scan_roots.as_deref());

        for root in configured_roots {
            scan_configured_root(
                &root,
                &now,
                options.include_plugin_caches,
                mcp_probe_policy,
                &mut roots,
                &mut entities,
                &mut issues,
            );
        }
        scan_instruction_files(
            home,
            &now,
            mcp_probe_policy,
            &mut roots,
            &mut entities,
            &mut issues,
        );
    }

    if let Ok(current_dir) = env::current_dir() {
        scan_instruction_files(
            &current_dir,
            &now,
            mcp_probe_policy,
            &mut roots,
            &mut entities,
            &mut issues,
        );
    }

    apply_duplicate_name_issues(&mut entities);
    let issues = aggregate_report_issues(&entities, issues);
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

fn configured_scan_roots(home: &Path, scan_roots: Option<&[String]>) -> Vec<PathBuf> {
    let roots = scan_roots
        .filter(|roots| !roots.is_empty())
        .map(|roots| roots.to_vec())
        .unwrap_or_else(default_scan_roots);

    roots
        .into_iter()
        .filter(|root| !root.trim().is_empty())
        .map(|root| expand_scan_root(home, &root))
        .collect()
}

fn default_scan_roots() -> Vec<String> {
    [
        "%USERPROFILE%\\.codex\\skills",
        "%USERPROFILE%\\.agents\\skills",
        "%USERPROFILE%\\.claude\\skills",
        "%USERPROFILE%\\.claude\\commands",
        "%USERPROFILE%\\.claude\\agents",
        "%USERPROFILE%\\.codex\\plugins",
        "%USERPROFILE%\\.claude\\plugins",
        "%USERPROFILE%\\.codex\\config.toml",
        "%USERPROFILE%\\.claude\\mcp-configs",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

fn expand_scan_root(home: &Path, root: &str) -> PathBuf {
    let home_text = home.to_string_lossy();
    let expanded = if root == "~" {
        home_text.to_string()
    } else if let Some(rest) = root.strip_prefix("~\\").or_else(|| root.strip_prefix("~/")) {
        format!("{}\\{}", home_text, rest)
    } else {
        root.to_string()
    };
    let expanded = expanded
        .replace("%USERPROFILE%", &home_text)
        .replace("%HOME%", &home_text);
    PathBuf::from(expanded)
}

fn scan_configured_root(
    root: &Path,
    discovered_at: &str,
    include_plugin_caches: bool,
    mcp_probe_policy: &str,
    roots: &mut Vec<Value>,
    entities: &mut Vec<Value>,
    issues: &mut Vec<Value>,
) {
    let normalized = normalized_path_text(root);

    if normalized.ends_with("\\.codex\\skills") {
        scan_skill_root(root, "codex", discovered_at, roots, entities, issues);
    } else if normalized.ends_with("\\.agents\\skills") {
        scan_skill_root(root, "shared", discovered_at, roots, entities, issues);
    } else if normalized.ends_with("\\.claude\\skills") {
        scan_skill_root(root, "claude-code", discovered_at, roots, entities, issues);
    } else if normalized.ends_with("\\.claude\\commands") {
        scan_markdown_root(root, "command", discovered_at, roots, entities, issues);
    } else if normalized.ends_with("\\.claude\\agents") {
        scan_markdown_root(root, "agent", discovered_at, roots, entities, issues);
    } else if normalized.ends_with("\\.codex\\plugins") {
        scan_plugin_root(
            root,
            "codex",
            discovered_at,
            include_plugin_caches,
            roots,
            entities,
            issues,
        );
    } else if normalized.ends_with("\\.claude\\plugins") {
        scan_plugin_root(
            root,
            "claude-code",
            discovered_at,
            include_plugin_caches,
            roots,
            entities,
            issues,
        );
    } else if normalized.ends_with("\\.codex\\config.toml") {
        scan_codex_config_toml(
            root,
            discovered_at,
            mcp_probe_policy,
            roots,
            entities,
            issues,
        );
    } else if normalized.ends_with("\\.claude\\mcp-configs") {
        scan_mcp_json_root(
            root,
            discovered_at,
            mcp_probe_policy,
            roots,
            entities,
            issues,
        );
    } else if is_instruction_file(root) {
        if !root.exists() {
            roots.push(root_result(root, "file", "missing", None));
        } else if !root.is_file() {
            roots.push(root_result(
                root,
                "file",
                "error",
                Some("Configured instruction path is not a file."),
            ));
        } else {
            roots.push(root_result(root, "file", "scanned", None));
            entities.push(build_instruction_entity(root, discovered_at));
            if root
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == ".mcp.json")
            {
                scan_mcp_json_file(root, discovered_at, mcp_probe_policy, entities, issues);
            }
        }
    } else {
        roots.push(root_result(
            root,
            if root.is_file() { "file" } else { "directory" },
            "skipped",
            Some("Scan root is not a supported SkillDesk MVP source."),
        ));
    }
}

fn normalized_path_text(path: &Path) -> String {
    path.to_string_lossy()
        .replace('/', "\\")
        .to_ascii_lowercase()
}

fn is_instruction_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "AGENTS.md" | "CLAUDE.md" | ".mcp.json"))
}

fn scan_plugin_root(
    root: &Path,
    platform: &str,
    discovered_at: &str,
    include_plugin_caches: bool,
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
            Some("Configured plugin root is not a directory."),
        ));
        return;
    }

    roots.push(root_result(root, "directory", "scanned", None));
    let mut manifests = Vec::new();
    collect_plugin_manifests(
        root,
        0,
        include_plugin_caches,
        &mut manifests,
        report_issues,
    );

    for manifest in manifests {
        if let Some(entity) =
            build_plugin_entity(root, platform, &manifest, discovered_at, report_issues)
        {
            entities.push(entity);
        }
    }
}

fn collect_plugin_manifests(
    dir: &Path,
    depth: usize,
    include_plugin_caches: bool,
    manifests: &mut Vec<PathBuf>,
    report_issues: &mut Vec<Value>,
) {
    if depth > MAX_MARKDOWN_SCAN_DEPTH || should_skip_path(dir) {
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

    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == "plugin.json") {
            manifests.push(path);
        } else if path.is_dir() && !should_skip_path(&path) {
            if !include_plugin_caches && is_plugin_cache_path(&path) {
                continue;
            }
            collect_plugin_manifests(
                &path,
                depth + 1,
                include_plugin_caches,
                manifests,
                report_issues,
            );
        }
    }
}

fn is_plugin_cache_path(path: &Path) -> bool {
    path_contains(path, "cache") || path_contains(path, "backups") || path_contains(path, "backup")
}

fn scan_instruction_files(
    start: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
    roots: &mut Vec<Value>,
    entities: &mut Vec<Value>,
    report_issues: &mut Vec<Value>,
) {
    let mut seen = HashSet::new();
    for dir in instruction_candidate_dirs(start) {
        for file_name in ["AGENTS.md", "CLAUDE.md", ".mcp.json"] {
            let file = dir.join(file_name);
            let stable_id = stable_path_id(&file);
            if !seen.insert(stable_id) {
                continue;
            }

            if file.exists() && file.is_file() {
                roots.push(root_result(&file, "file", "scanned", None));
                entities.push(build_instruction_entity(&file, discovered_at));
                if file_name == ".mcp.json" {
                    scan_mcp_json_file(
                        &file,
                        discovered_at,
                        mcp_probe_policy,
                        entities,
                        report_issues,
                    );
                }
            }
        }
    }
}

fn instruction_candidate_dirs(start: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut current = Some(start);

    for _ in 0..=4 {
        let Some(dir) = current else {
            break;
        };
        dirs.push(dir.to_path_buf());
        current = dir.parent();
    }

    dirs
}

fn scan_markdown_root(
    root: &Path,
    entity_kind: &str,
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
            Some("Configured markdown root is not a directory."),
        ));
        return;
    }

    roots.push(root_result(root, "directory", "scanned", None));
    let mut markdown_files = Vec::new();
    collect_markdown_files(root, 0, &mut markdown_files, report_issues);

    for file in markdown_files {
        let entity = match entity_kind {
            "command" => build_command_entity(root, &file, discovered_at),
            "agent" => build_agent_entity(root, &file, discovered_at),
            _ => continue,
        };
        entities.push(entity);
    }
}

fn collect_markdown_files(
    dir: &Path,
    depth: usize,
    markdown_files: &mut Vec<PathBuf>,
    report_issues: &mut Vec<Value>,
) {
    if depth > MAX_MARKDOWN_SCAN_DEPTH || should_skip_path(dir) {
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

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !should_skip_path(&path) {
            collect_markdown_files(&path, depth + 1, markdown_files, report_issues);
        } else if has_extension(&path, "md") {
            markdown_files.push(path);
        }
    }
}

fn scan_mcp_json_root(
    root: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
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
            Some("Configured MCP root is not a directory."),
        ));
        return;
    }

    roots.push(root_result(root, "directory", "scanned", None));
    let mut json_files = Vec::new();
    collect_json_files(root, 0, &mut json_files, report_issues);

    for file in json_files {
        scan_mcp_json_file(
            &file,
            discovered_at,
            mcp_probe_policy,
            entities,
            report_issues,
        );
    }
}

fn collect_json_files(
    dir: &Path,
    depth: usize,
    json_files: &mut Vec<PathBuf>,
    report_issues: &mut Vec<Value>,
) {
    if depth > MAX_MARKDOWN_SCAN_DEPTH || should_skip_path(dir) {
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

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !should_skip_path(&path) {
            collect_json_files(&path, depth + 1, json_files, report_issues);
        } else if has_extension(&path, "json") {
            json_files.push(path);
        }
    }
}

fn scan_mcp_json_file(
    file: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
    entities: &mut Vec<Value>,
    report_issues: &mut Vec<Value>,
) {
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("mcp-read", file),
              "severity": "low",
              "category": "mcp",
              "message": "SkillDesk could not read an MCP config file.",
              "file": file.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return;
        }
    };

    let parsed: Value = match serde_json::from_str(&content) {
        Ok(parsed) => parsed,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("mcp-json", file),
              "severity": "medium",
              "category": "mcp",
              "message": "MCP config JSON could not be parsed.",
              "file": file.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return;
        }
    };

    let Some(servers) = parsed.get("mcpServers").and_then(Value::as_object) else {
        return;
    };

    for (name, config) in servers {
        entities.push(build_mcp_server_entity(
            name,
            config,
            file,
            discovered_at,
            mcp_probe_policy,
        ));
    }
}

fn scan_codex_config_toml(
    file: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
    roots: &mut Vec<Value>,
    entities: &mut Vec<Value>,
    report_issues: &mut Vec<Value>,
) {
    if !file.exists() {
        roots.push(root_result(file, "file", "missing", None));
        return;
    }

    if !file.is_file() {
        roots.push(root_result(
            file,
            "file",
            "error",
            Some("Configured Codex config path is not a file."),
        ));
        return;
    }

    roots.push(root_result(file, "file", "scanned", None));
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("codex-config-read", file),
              "severity": "low",
              "category": "path",
              "message": "SkillDesk could not read Codex config.toml.",
              "file": file.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return;
        }
    };

    let parsed: toml::Value = match toml::from_str(&content) {
        Ok(parsed) => parsed,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("codex-config-toml", file),
              "severity": "medium",
              "category": "format",
              "message": "Codex config.toml could not be parsed.",
              "file": file.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return;
        }
    };

    for (server_name, server_config) in codex_mcp_tables(&parsed) {
        entities.push(build_codex_mcp_server_entity(
            &server_name,
            &server_config,
            file,
            discovered_at,
            mcp_probe_policy,
        ));
    }
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

fn build_command_entity(root: &Path, file: &Path, discovered_at: &str) -> Value {
    let name = file_stem(file, "command");
    let content = fs::read_to_string(file).unwrap_or_default();
    let (title, description) = parse_markdown_text(&content);
    let health = markdown_health("command", &name, file, &content, description.as_deref());
    let mut entity = base_entity(
        "command",
        "claude-code",
        &name,
        root,
        file,
        discovered_at,
        title,
        description,
        health,
    );
    entity.insert("commandType".to_string(), json!("slash-command"));
    entity.insert("file".to_string(), json!(file.to_string_lossy()));
    if let Some(namespace) = namespace_for(root, file) {
        entity.insert("namespace".to_string(), json!(namespace));
    }
    Value::Object(entity)
}

fn build_agent_entity(root: &Path, file: &Path, discovered_at: &str) -> Value {
    let name = file_stem(file, "agent");
    let content = fs::read_to_string(file).unwrap_or_default();
    let (title, description) = parse_markdown_text(&content);
    let health = markdown_health("agent", &name, file, &content, description.as_deref());
    let mut entity = base_entity(
        "agent",
        "claude-code",
        &name,
        root,
        file,
        discovered_at,
        title,
        description,
        health,
    );
    entity.insert("file".to_string(), json!(file.to_string_lossy()));
    entity.insert(
        "declaredTools".to_string(),
        json!(parse_declared_tools(&content)),
    );
    if let Some(model) = parse_declared_field(&content, "model") {
        entity.insert("declaredModel".to_string(), json!(model));
    }
    Value::Object(entity)
}

fn build_instruction_entity(file: &Path, discovered_at: &str) -> Value {
    let name = file
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "instruction-file".to_string());
    let content = fs::read_to_string(file).unwrap_or_default();
    let (title, description) = parse_markdown_text(&content);
    let health = instruction_health(&name, file, &content);
    let applies_to_path = file
        .parent()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut entity = base_entity(
        "instruction-file",
        "shared",
        &name,
        file.parent().unwrap_or(file),
        file,
        discovered_at,
        title,
        description,
        health,
    );
    entity.insert("fileType".to_string(), json!(instruction_file_type(&name)));
    entity.insert("appliesToPath".to_string(), json!(applies_to_path));
    entity.insert("lineCount".to_string(), json!(content.lines().count()));
    Value::Object(entity)
}

fn build_plugin_entity(
    plugin_root: &Path,
    platform: &str,
    manifest_path: &Path,
    discovered_at: &str,
    report_issues: &mut Vec<Value>,
) -> Option<Value> {
    let content = match fs::read_to_string(manifest_path) {
        Ok(content) => content,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("plugin-read", manifest_path),
              "severity": "low",
              "category": "format",
              "message": "SkillDesk could not read a plugin manifest.",
              "file": manifest_path.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return None;
        }
    };
    let manifest: Value = match serde_json::from_str(&content) {
        Ok(manifest) => manifest,
        Err(error) => {
            report_issues.push(json!({
              "id": issue_id("plugin-json", manifest_path),
              "severity": "medium",
              "category": "format",
              "message": "Plugin manifest JSON could not be parsed.",
              "file": manifest_path.to_string_lossy(),
              "evidence": error.to_string(),
            }));
            return None;
        }
    };
    let plugin_dir = manifest_path.parent().unwrap_or(plugin_root);
    let fallback_name = plugin_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "plugin".to_string());
    let name = manifest
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(&fallback_name)
        .to_string();
    let health = plugin_health(&name, manifest_path, &manifest);
    let is_cache = path_contains(plugin_dir, "cache");
    let mut entity = base_entity(
        "plugin",
        platform,
        &name,
        plugin_root,
        manifest_path,
        discovered_at,
        manifest
            .get("title")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        manifest
            .get("description")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        health,
    );
    entity.insert("path".to_string(), json!(plugin_dir.to_string_lossy()));
    entity.insert(
        "source".to_string(),
        json!(if is_cache { "plugin-cache" } else { "local" }),
    );
    entity.insert(
        "manifestPath".to_string(),
        json!(manifest_path.to_string_lossy()),
    );
    if let Some(version) = manifest.get("version").and_then(Value::as_str) {
        entity.insert("version".to_string(), json!(version));
    }
    if let Some(publisher) = manifest.get("publisher").and_then(Value::as_str) {
        entity.insert("publisher".to_string(), json!(publisher));
    }
    entity.insert(
        "bundled".to_string(),
        json!({
          "skills": manifest_count(&manifest, "skills").unwrap_or_else(|| child_count(plugin_dir, "skills")),
          "commands": manifest_count(&manifest, "commands").unwrap_or_else(|| child_count(plugin_dir, "commands")),
          "agents": manifest_count(&manifest, "agents").unwrap_or_else(|| child_count(plugin_dir, "agents")),
          "mcpServers": manifest_count(&manifest, "mcpServers").unwrap_or_default(),
          "hooks": manifest_count(&manifest, "hooks").unwrap_or_default(),
        }),
    );
    let mut cache = Map::new();
    cache.insert("isCache".to_string(), json!(is_cache));
    cache.insert(
        "isBackup".to_string(),
        json!(path_contains(plugin_dir, "backup") || path_contains(plugin_dir, "backups")),
    );
    if let Some(cache_family) = plugin_dir
        .parent()
        .and_then(Path::file_name)
        .map(|value| value.to_string_lossy().to_string())
    {
        cache.insert("cacheFamily".to_string(), json!(cache_family));
    }
    entity.insert("cache".to_string(), Value::Object(cache));
    Some(Value::Object(entity))
}

fn build_mcp_server_entity(
    name: &str,
    config: &Value,
    config_path: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
) -> Value {
    let command = config
        .get("command")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let url = config.get("url").and_then(Value::as_str);
    let transport = mcp_transport(config, command.as_deref(), url);
    let probe = mcp_probe_result(mcp_probe_policy, url);
    let probe_error = probe
        .get("reachable")
        .and_then(Value::as_bool)
        .is_some_and(|reachable| !reachable)
        .then(|| {
            probe
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("MCP URL was not reachable.")
        });
    let health = mcp_health(name, config_path, command.as_deref(), url, probe_error);
    let mut entity = base_entity(
        "mcp-server",
        "unknown",
        name,
        config_path.parent().unwrap_or(config_path),
        config_path,
        discovered_at,
        None,
        None,
        health,
    );
    entity.insert(
        "configPath".to_string(),
        json!(config_path.to_string_lossy()),
    );
    entity.insert("transport".to_string(), json!(transport));
    entity.insert("probe".to_string(), probe);
    if let Some(value) = command {
        entity.insert("command".to_string(), json!(value));
    }
    if let Some(args_count) = config.get("args").and_then(Value::as_array).map(Vec::len) {
        entity.insert("argsCount".to_string(), json!(args_count));
    }
    if let Some(host) = url.and_then(url_host) {
        entity.insert("urlHost".to_string(), json!(host));
    }
    Value::Object(entity)
}

fn build_codex_mcp_server_entity(
    name: &str,
    config: &toml::Value,
    config_path: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
) -> Value {
    let command = toml_string(config, "command");
    let url = toml_string(config, "url");
    let transport = codex_mcp_transport(config, command.as_deref(), url.as_deref());
    let probe = mcp_probe_result(mcp_probe_policy, url.as_deref());
    let probe_error = probe
        .get("reachable")
        .and_then(Value::as_bool)
        .is_some_and(|reachable| !reachable)
        .then(|| {
            probe
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("MCP URL was not reachable.")
        });
    let health = mcp_health(
        name,
        config_path,
        command.as_deref(),
        url.as_deref(),
        probe_error,
    );
    let mut entity = base_entity(
        "mcp-server",
        "codex",
        name,
        config_path.parent().unwrap_or(config_path),
        config_path,
        discovered_at,
        None,
        None,
        health,
    );
    entity.insert(
        "configPath".to_string(),
        json!(config_path.to_string_lossy()),
    );
    entity.insert("transport".to_string(), json!(transport));
    entity.insert("probe".to_string(), probe);
    if let Some(value) = command {
        entity.insert("command".to_string(), json!(value));
    }
    if let Some(args_count) = config
        .get("args")
        .and_then(toml::Value::as_array)
        .map(Vec::len)
    {
        entity.insert("argsCount".to_string(), json!(args_count));
    }
    if let Some(host) = url.as_deref().and_then(url_host) {
        entity.insert("urlHost".to_string(), json!(host));
    }
    Value::Object(entity)
}

fn mcp_probe_result(policy: &str, url: Option<&str>) -> Value {
    if policy == "disabled" {
        return json!({ "attempted": false });
    }

    let Some(url) = url else {
        return json!({ "attempted": false });
    };
    let Some((host, port)) = url_host_port(url) else {
        return json!({
          "attempted": true,
          "reachable": false,
          "error": "MCP URL host or port could not be parsed.",
        });
    };

    if policy == "local-only" && !is_local_host(&host) {
        return json!({ "attempted": false });
    }

    let start = Instant::now();
    let address = format!("{}:{}", host, port);
    let reachable = address
        .to_socket_addrs()
        .ok()
        .and_then(|mut addresses| {
            addresses.find_map(|socket_address| {
                TcpStream::connect_timeout(&socket_address, Duration::from_millis(250)).ok()
            })
        })
        .is_some();
    let latency_ms = start.elapsed().as_millis() as u64;

    if reachable {
        json!({
          "attempted": true,
          "reachable": true,
          "latencyMs": latency_ms,
        })
    } else {
        json!({
          "attempted": true,
          "reachable": false,
          "latencyMs": latency_ms,
          "error": "MCP URL TCP endpoint was not reachable.",
        })
    }
}

fn url_host_port(url: &str) -> Option<(String, u16)> {
    let scheme = url.split_once("://").map(|(scheme, _)| scheme)?;
    let host = url_host(url)?;
    if let Some((host, port)) = host.rsplit_once(':') {
        let port = port.parse::<u16>().ok()?;
        let host = host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_string();
        return Some((host, port));
    }

    let port = match scheme.to_ascii_lowercase().as_str() {
        "http" => 80,
        "https" => 443,
        _ => return None,
    };
    Some((host, port))
}

fn is_local_host(host: &str) -> bool {
    matches!(
        host.to_ascii_lowercase().as_str(),
        "localhost" | "127.0.0.1" | "::1" | "[::1]"
    )
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
    attach_git_state(&mut entity, skill_dir);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        net::TcpListener,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn plugin_cache_scan_is_controlled_by_options() {
        let root = unique_test_dir("plugin-cache-scan");
        let local_plugin = root.join("local-plugin");
        let cached_plugin = root.join("cache").join("cached-plugin");
        fs::create_dir_all(&local_plugin).expect("create local plugin dir");
        fs::create_dir_all(&cached_plugin).expect("create cached plugin dir");
        fs::write(
            local_plugin.join("plugin.json"),
            r#"{"name":"local-plugin","description":"Local plugin."}"#,
        )
        .expect("write local plugin manifest");
        fs::write(
            cached_plugin.join("plugin.json"),
            r#"{"name":"cached-plugin","description":"Cached plugin."}"#,
        )
        .expect("write cached plugin manifest");

        let mut roots = Vec::new();
        let mut entities = Vec::new();
        let mut issues = Vec::new();
        scan_plugin_root(
            &root,
            "codex",
            "2026-06-07T00:00:00Z",
            false,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0]["name"], "local-plugin");

        roots.clear();
        entities.clear();
        issues.clear();
        scan_plugin_root(
            &root,
            "codex",
            "2026-06-07T00:00:00Z",
            true,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        let names: Vec<_> = entities
            .iter()
            .filter_map(|entity| entity["name"].as_str())
            .collect();
        assert!(names.contains(&"local-plugin"));
        assert!(names.contains(&"cached-plugin"));

        fs::remove_dir_all(root).expect("remove plugin scan test dir");
    }

    #[test]
    fn configured_claude_plugin_root_sets_platform() {
        let home = unique_test_dir("configured-roots");
        let root = home.join(".claude").join("plugins");
        let plugin = root.join("demo-plugin");
        fs::create_dir_all(&plugin).expect("create claude plugin dir");
        fs::write(
            plugin.join("plugin.json"),
            r#"{"name":"demo-plugin","description":"Claude plugin."}"#,
        )
        .expect("write claude plugin manifest");

        let mut roots = Vec::new();
        let mut entities = Vec::new();
        let mut issues = Vec::new();
        scan_configured_root(
            &root,
            "2026-06-07T00:00:00Z",
            false,
            "disabled",
            &mut roots,
            &mut entities,
            &mut issues,
        );

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0]["kind"], "plugin");
        assert_eq!(entities[0]["platform"], "claude-code");
        fs::remove_dir_all(home).expect("remove configured roots test dir");
    }

    #[test]
    fn configured_agents_skill_root_sets_shared_platform() {
        let home = unique_test_dir("configured-agents-root");
        let root = home.join(".agents").join("skills");
        let skill = root.join("shared-skill");
        fs::create_dir_all(&skill).expect("create shared skill dir");
        fs::write(
            skill.join("SKILL.md"),
            "# Shared Skill\nUse when testing shared skill discovery.",
        )
        .expect("write shared skill");

        let mut roots = Vec::new();
        let mut entities = Vec::new();
        let mut issues = Vec::new();
        scan_configured_root(
            &root,
            "2026-06-07T00:00:00Z",
            false,
            "disabled",
            &mut roots,
            &mut entities,
            &mut issues,
        );

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0]["kind"], "skill");
        assert_eq!(entities[0]["platform"], "shared");
        fs::remove_dir_all(home).expect("remove configured agents root test dir");
    }

    #[test]
    fn unsupported_configured_root_is_skipped() {
        let root = unique_test_dir("unsupported-root");
        let mut roots = Vec::new();
        let mut entities = Vec::new();
        let mut issues = Vec::new();

        scan_configured_root(
            &root,
            "2026-06-07T00:00:00Z",
            false,
            "disabled",
            &mut roots,
            &mut entities,
            &mut issues,
        );

        assert_eq!(entities.len(), 0);
        assert_eq!(roots[0]["status"], "skipped");
    }

    #[test]
    fn scan_local_extensions_reads_configured_fixture_roots() {
        let home = unique_test_dir("local-scan-fixture");
        let codex_skill = home.join(".codex").join("skills").join("review-skill");
        let claude_command = home.join(".claude").join("commands");
        let claude_agent = home.join(".claude").join("agents");
        let codex_plugin = home.join(".codex").join("plugins").join("demo-plugin");
        let mcp_configs = home.join(".claude").join("mcp-configs");

        fs::create_dir_all(&codex_skill).expect("create skill dir");
        fs::create_dir_all(&claude_command).expect("create command dir");
        fs::create_dir_all(&claude_agent).expect("create agent dir");
        fs::create_dir_all(&codex_plugin).expect("create plugin dir");
        fs::create_dir_all(&mcp_configs).expect("create mcp config dir");

        fs::write(
            codex_skill.join("SKILL.md"),
            "# Review Skill\nUse when reviewing local scanner fixture metadata.",
        )
        .expect("write skill");
        fs::write(
            claude_command.join("review.md"),
            "# Review Command\nUse when testing command discovery.",
        )
        .expect("write command");
        fs::write(
            claude_agent.join("reviewer.md"),
            "# Reviewer Agent\nUse when testing agent discovery.\n\nmodel: sonnet\ntools: Read, Grep",
        )
        .expect("write agent");
        fs::write(
            codex_plugin.join("plugin.json"),
            r#"{"name":"demo-plugin","description":"Plugin fixture.","version":"0.1.0"}"#,
        )
        .expect("write plugin");
        fs::write(
            mcp_configs.join("demo.json"),
            r#"{"mcpServers":{"demo":{"command":"node","args":["server.js"]}}}"#,
        )
        .expect("write mcp config");

        let report = scan_local_extensions(Some(ScanOptions {
            include_plugin_caches: false,
            mcp_probe_policy: Some("disabled".to_string()),
            scan_roots: Some(vec![
                home.join(".codex")
                    .join("skills")
                    .to_string_lossy()
                    .to_string(),
                home.join(".claude")
                    .join("commands")
                    .to_string_lossy()
                    .to_string(),
                home.join(".claude")
                    .join("agents")
                    .to_string_lossy()
                    .to_string(),
                home.join(".codex")
                    .join("plugins")
                    .to_string_lossy()
                    .to_string(),
                home.join(".claude")
                    .join("mcp-configs")
                    .to_string_lossy()
                    .to_string(),
            ]),
        }));

        assert_eq!(report["schemaVersion"], "0.1");
        assert!(report["totals"]["skills"].as_u64().unwrap_or_default() >= 1);
        assert!(report["totals"]["commands"].as_u64().unwrap_or_default() >= 1);
        assert!(report["totals"]["agents"].as_u64().unwrap_or_default() >= 1);
        assert!(report["totals"]["plugins"].as_u64().unwrap_or_default() >= 1);
        assert!(report["totals"]["mcpServers"].as_u64().unwrap_or_default() >= 1);

        fs::remove_dir_all(home).expect("remove local scan fixture dir");
    }

    #[test]
    fn mcp_probe_reaches_local_tcp_endpoint() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test listener");
        let port = listener
            .local_addr()
            .expect("read local listener address")
            .port();
        let probe = mcp_probe_result("local-only", Some(&format!("http://127.0.0.1:{port}/mcp")));

        assert_eq!(probe["attempted"], true);
        assert_eq!(probe["reachable"], true);
    }

    #[test]
    fn local_only_mcp_probe_skips_remote_hosts() {
        let probe = mcp_probe_result("local-only", Some("https://example.invalid/mcp"));

        assert_eq!(probe["attempted"], false);
        assert!(probe.get("reachable").is_none());
    }

    fn unique_test_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        env::temp_dir().join(format!("skilldesk-{name}-{}-{nanos}", std::process::id()))
    }
}

fn parse_skill_text(content: &str) -> (Option<String>, Option<String>) {
    parse_markdown_text(content)
}

fn parse_markdown_text(content: &str) -> (Option<String>, Option<String>) {
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

fn base_entity(
    kind: &str,
    platform: &str,
    name: &str,
    root: &Path,
    file: &Path,
    discovered_at: &str,
    title: Option<String>,
    description: Option<String>,
    health: Value,
) -> Map<String, Value> {
    let path = if kind == "skill" {
        file.parent().unwrap_or(root)
    } else {
        file
    };
    let mut entity = Map::new();
    entity.insert(
        "id".to_string(),
        json!(format!("{}:{}:{}", kind, platform, stable_path_id(path))),
    );
    entity.insert("kind".to_string(), json!(kind));
    entity.insert("platform".to_string(), json!(platform));
    entity.insert("name".to_string(), json!(name));
    if let Some(value) = title {
        entity.insert("title".to_string(), json!(value));
    }
    if let Some(value) = description {
        entity.insert("description".to_string(), json!(value));
    }
    entity.insert("path".to_string(), json!(path.to_string_lossy()));
    entity.insert("source".to_string(), json!("local"));
    entity.insert("tags".to_string(), json!([]));
    entity.insert("discoveredAt".to_string(), json!(discovered_at));
    let (last_modified, fingerprint) = file_snapshot(file);
    if let Some(value) = last_modified {
        entity.insert("lastModified".to_string(), json!(value));
    }
    if let Some(value) = fingerprint {
        entity.insert("fingerprint".to_string(), json!(value));
    }
    entity.insert("health".to_string(), health);
    attach_git_state(&mut entity, path);
    entity
}
