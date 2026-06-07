use serde_json::{json, Map, Value};
use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

const SCHEMA_VERSION: &str = "0.1";
const MAX_SKILL_SCAN_DEPTH: usize = 8;
const MAX_MARKDOWN_SCAN_DEPTH: usize = 8;

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

        scan_plugin_root(
            &home.join(".codex").join("plugins"),
            &now,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        scan_codex_config_toml(
            &home.join(".codex").join("config.toml"),
            &now,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        scan_markdown_root(
            &home.join(".claude").join("commands"),
            "command",
            &now,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        scan_markdown_root(
            &home.join(".claude").join("agents"),
            "agent",
            &now,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        scan_mcp_json_root(
            &home.join(".claude").join("mcp-configs"),
            &now,
            &mut roots,
            &mut entities,
            &mut issues,
        );
        scan_instruction_files(home, &now, &mut roots, &mut entities, &mut issues);
    }

    if let Ok(current_dir) = env::current_dir() {
        scan_instruction_files(&current_dir, &now, &mut roots, &mut entities, &mut issues);
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

fn scan_plugin_root(
    root: &Path,
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
            Some("Configured plugin root is not a directory."),
        ));
        return;
    }

    roots.push(root_result(root, "directory", "scanned", None));
    let mut manifests = Vec::new();
    collect_plugin_manifests(root, 0, &mut manifests, report_issues);

    for manifest in manifests {
        if let Some(entity) = build_plugin_entity(root, &manifest, discovered_at, report_issues) {
            entities.push(entity);
        }
    }
}

fn collect_plugin_manifests(
    dir: &Path,
    depth: usize,
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
            collect_plugin_manifests(&path, depth + 1, manifests, report_issues);
        }
    }
}

fn scan_instruction_files(
    start: &Path,
    discovered_at: &str,
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
                    scan_mcp_json_file(&file, discovered_at, entities, report_issues);
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
        scan_mcp_json_file(&file, discovered_at, entities, report_issues);
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
        entities.push(build_mcp_server_entity(name, config, file, discovered_at));
    }
}

fn scan_codex_config_toml(
    file: &Path,
    discovered_at: &str,
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
        "codex",
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
) -> Value {
    let command = config
        .get("command")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let url = config.get("url").and_then(Value::as_str);
    let transport = mcp_transport(config, command.as_deref(), url);
    let health = mcp_health(name, config_path, command.as_deref(), url);
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
    entity.insert(
        "probe".to_string(),
        json!({
          "attempted": false,
        }),
    );
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
) -> Value {
    let command = toml_string(config, "command");
    let url = toml_string(config, "url");
    let transport = codex_mcp_transport(config, command.as_deref(), url.as_deref());
    let health = mcp_health(name, config_path, command.as_deref(), url.as_deref());
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
    entity.insert(
        "probe".to_string(),
        json!({
          "attempted": false,
        }),
    );
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
    if let Some(value) = modified_iso(file) {
        entity.insert("lastModified".to_string(), json!(value));
    }
    if let Some(value) = file_fingerprint(file) {
        entity.insert("fingerprint".to_string(), json!(value));
    }
    entity.insert("health".to_string(), health);
    entity
}

fn instruction_health(name: &str, file: &Path, content: &str) -> Value {
    let mut issues = Vec::new();

    if content.trim().is_empty() {
        issues.push(json!({
          "id": issue_id("empty-instruction", file),
          "severity": "medium",
          "category": "format",
          "message": format!("Instruction file '{}' is empty.", name),
          "file": file.to_string_lossy(),
          "recommendation": "Add concise project guidance or remove the empty file.",
        }));
    }

    if content.len() > 200_000 {
        issues.push(json!({
          "id": issue_id("large-instruction", file),
          "severity": "low",
          "category": "size",
          "message": format!("Instruction file '{}' is unusually large.", name),
          "file": file.to_string_lossy(),
          "recommendation": "Keep project instruction files focused so agents can load them reliably.",
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

fn plugin_health(name: &str, manifest_path: &Path, manifest: &Value) -> Value {
    let mut issues = Vec::new();

    if manifest.get("name").and_then(Value::as_str).is_none() {
        issues.push(json!({
          "id": issue_id("plugin-name", manifest_path),
          "severity": "low",
          "category": "metadata",
          "message": format!("Plugin '{}' has no manifest name.", name),
          "file": manifest_path.to_string_lossy(),
          "recommendation": "Add a stable name field to plugin.json.",
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

fn markdown_health(
    kind: &str,
    name: &str,
    file: &Path,
    content: &str,
    description: Option<&str>,
) -> Value {
    let mut issues = Vec::new();

    if content.trim().is_empty() {
        issues.push(json!({
          "id": issue_id("empty-markdown", file),
          "severity": "medium",
          "category": "format",
          "message": format!("{} '{}' is empty.", kind, name),
          "file": file.to_string_lossy(),
          "recommendation": "Add a concise description and usage instructions.",
        }));
    }

    if description.is_none_or(|value| value.chars().count() < 16) {
        issues.push(json!({
          "id": issue_id("weak-description", file),
          "severity": "low",
          "category": "metadata",
          "message": format!("{} '{}' has a weak or missing description.", kind, name),
          "file": file.to_string_lossy(),
          "recommendation": "Add a clear summary explaining when this file should be used.",
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

fn mcp_health(name: &str, config_path: &Path, command: Option<&str>, url: Option<&str>) -> Value {
    let mut issues = Vec::new();

    if command.is_none() && url.is_none() {
        issues.push(json!({
          "id": issue_id("mcp-missing-launch", config_path),
          "severity": "medium",
          "category": "mcp",
          "message": format!("MCP server '{}' has neither command nor url.", name),
          "file": config_path.to_string_lossy(),
          "recommendation": "Define a stdio command or an HTTP/SSE URL in the MCP config.",
        }));
    }

    let status = if issues.is_empty() {
        "needs-review"
    } else {
        "broken"
    };

    json!({
      "status": status,
      "issues": issues,
    })
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

fn manifest_count(manifest: &Value, key: &str) -> Option<usize> {
    let value = manifest.get(key)?;
    if let Some(items) = value.as_array() {
        return Some(items.len());
    }
    if let Some(items) = value.as_object() {
        return Some(items.len());
    }
    None
}

fn child_count(root: &Path, child_name: &str) -> usize {
    let path = root.join(child_name);
    if !path.exists() || !path.is_dir() {
        return 0;
    }

    fs::read_dir(path)
        .map(|entries| entries.flatten().count())
        .unwrap_or_default()
}

fn path_contains(path: &Path, needle: &str) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|value| value.eq_ignore_ascii_case(needle))
    })
}

fn mcp_transport(config: &Value, command: Option<&str>, url: Option<&str>) -> &'static str {
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

fn codex_mcp_tables(parsed: &toml::Value) -> Vec<(String, toml::Value)> {
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

fn codex_mcp_transport(
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

fn toml_string(config: &toml::Value, key: &str) -> Option<String> {
    config
        .get(key)
        .and_then(toml::Value::as_str)
        .map(ToString::to_string)
}

fn url_host(url: &str) -> Option<String> {
    let without_scheme = url.split_once("://").map(|(_, rest)| rest).unwrap_or(url);
    without_scheme
        .split('/')
        .next()
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn instruction_file_type(name: &str) -> &str {
    match name {
        "AGENTS.md" => "AGENTS.md",
        "CLAUDE.md" => "CLAUDE.md",
        ".mcp.json" => ".mcp.json",
        _ => "other",
    }
}

fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

fn file_stem(path: &Path, fallback: &str) -> String {
    path.file_stem()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| fallback.to_string())
}

fn namespace_for(root: &Path, file: &Path) -> Option<String> {
    let parent = file.parent()?;
    let relative = parent.strip_prefix(root).ok()?;
    let namespace = relative.to_string_lossy().replace('\\', "/");
    if namespace.is_empty() {
        None
    } else {
        Some(namespace)
    }
}

fn parse_declared_tools(content: &str) -> Vec<String> {
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

fn parse_declared_field(content: &str, key: &str) -> Option<String> {
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
