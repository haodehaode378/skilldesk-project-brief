use super::{
    common::*,
    git::{attach_git_state, GitStateCache},
    health::*,
    mcp::mcp_probe_result,
};
use serde_json::{json, Map, Value};
use std::{fs, path::Path};

pub(crate) fn build_command_entity(
    root: &Path,
    file: &Path,
    discovered_at: &str,
    git_cache: &mut GitStateCache,
) -> Value {
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
        git_cache,
    );
    entity.insert("commandType".to_string(), json!("slash-command"));
    entity.insert("file".to_string(), json!(file.to_string_lossy()));
    if let Some(namespace) = namespace_for(root, file) {
        entity.insert("namespace".to_string(), json!(namespace));
    }
    Value::Object(entity)
}

pub(crate) fn build_agent_entity(
    root: &Path,
    file: &Path,
    discovered_at: &str,
    git_cache: &mut GitStateCache,
) -> Value {
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
        git_cache,
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

pub(crate) fn build_instruction_entity(
    file: &Path,
    discovered_at: &str,
    git_cache: &mut GitStateCache,
) -> Value {
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
        git_cache,
    );
    entity.insert("fileType".to_string(), json!(instruction_file_type(&name)));
    entity.insert("appliesToPath".to_string(), json!(applies_to_path));
    entity.insert("lineCount".to_string(), json!(content.lines().count()));
    Value::Object(entity)
}

pub(crate) fn build_plugin_entity(
    plugin_root: &Path,
    platform: &str,
    manifest_path: &Path,
    discovered_at: &str,
    report_issues: &mut Vec<Value>,
    git_cache: &mut GitStateCache,
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
        git_cache,
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

pub(crate) fn build_mcp_server_entity(
    name: &str,
    config: &Value,
    config_path: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
    git_cache: &mut GitStateCache,
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
        git_cache,
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

pub(crate) fn build_codex_mcp_server_entity(
    name: &str,
    config: &toml::Value,
    config_path: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
    git_cache: &mut GitStateCache,
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
        git_cache,
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

pub(crate) fn build_skill_entity(
    root: &Path,
    skill_md: &Path,
    platform: &str,
    discovered_at: &str,
    git_cache: &mut GitStateCache,
) -> Value {
    let skill_dir = skill_md.parent().unwrap_or(root);
    let name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "skill".to_string());
    let content = fs::read_to_string(skill_md).unwrap_or_default();
    let (title, description) = parse_skill_text(&content);
    let health = skill_health(&name, skill_md, &content, description.as_deref());
    let mut entity = base_entity(
        "skill",
        platform,
        &name,
        root,
        skill_md,
        discovered_at,
        title,
        description,
        health,
        git_cache,
    );
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
    git_cache: &mut GitStateCache,
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
    attach_git_state(&mut entity, path, git_cache);
    entity
}
