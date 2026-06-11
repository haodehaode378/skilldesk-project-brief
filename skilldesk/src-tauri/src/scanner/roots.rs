use super::{common::*, entities::*, git::GitStateCache};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

const MAX_SKILL_SCAN_DEPTH: usize = 8;
const MAX_MARKDOWN_SCAN_DEPTH: usize = 8;

pub(crate) fn scan_configured_roots(
    home: &Path,
    scan_roots: Option<&[String]>,
    discovered_at: &str,
    include_plugin_caches: bool,
    mcp_probe_policy: &str,
    roots: &mut Vec<Value>,
    entities: &mut Vec<Value>,
    issues: &mut Vec<Value>,
    git_cache: &mut GitStateCache,
) {
    for root in configured_scan_roots(home, scan_roots) {
        scan_configured_root(
            &root,
            discovered_at,
            include_plugin_caches,
            mcp_probe_policy,
            roots,
            entities,
            issues,
            git_cache,
        );
    }
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
    git_cache: &mut GitStateCache,
) {
    let normalized = normalized_path_text(root);

    if normalized.ends_with("\\.codex\\skills") {
        scan_skill_root(
            root,
            "codex",
            discovered_at,
            roots,
            entities,
            issues,
            git_cache,
        );
    } else if normalized.ends_with("\\.agents\\skills") {
        scan_skill_root(
            root,
            "shared",
            discovered_at,
            roots,
            entities,
            issues,
            git_cache,
        );
    } else if normalized.ends_with("\\.claude\\skills") {
        scan_skill_root(
            root,
            "claude-code",
            discovered_at,
            roots,
            entities,
            issues,
            git_cache,
        );
    } else if normalized.ends_with("\\.claude\\commands") {
        scan_markdown_root(
            root,
            "command",
            discovered_at,
            roots,
            entities,
            issues,
            git_cache,
        );
    } else if normalized.ends_with("\\.claude\\agents") {
        scan_markdown_root(
            root,
            "agent",
            discovered_at,
            roots,
            entities,
            issues,
            git_cache,
        );
    } else if normalized.ends_with("\\.codex\\plugins") {
        scan_plugin_root(
            root,
            "codex",
            discovered_at,
            include_plugin_caches,
            roots,
            entities,
            issues,
            git_cache,
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
            git_cache,
        );
    } else if normalized.ends_with("\\.codex\\config.toml") {
        scan_codex_config_toml(
            root,
            discovered_at,
            mcp_probe_policy,
            roots,
            entities,
            issues,
            git_cache,
        );
    } else if normalized.ends_with("\\.claude\\mcp-configs") {
        scan_mcp_json_root(
            root,
            discovered_at,
            mcp_probe_policy,
            roots,
            entities,
            issues,
            git_cache,
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
            entities.push(build_instruction_entity(root, discovered_at, git_cache));
            if root
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == ".mcp.json")
            {
                scan_mcp_json_file(
                    root,
                    discovered_at,
                    mcp_probe_policy,
                    entities,
                    issues,
                    git_cache,
                );
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
    git_cache: &mut GitStateCache,
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
        if let Some(entity) = build_plugin_entity(
            root,
            platform,
            &manifest,
            discovered_at,
            report_issues,
            git_cache,
        ) {
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

pub(crate) fn scan_instruction_files(
    start: &Path,
    discovered_at: &str,
    mcp_probe_policy: &str,
    roots: &mut Vec<Value>,
    entities: &mut Vec<Value>,
    report_issues: &mut Vec<Value>,
    git_cache: &mut GitStateCache,
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
                entities.push(build_instruction_entity(&file, discovered_at, git_cache));
                if file_name == ".mcp.json" {
                    scan_mcp_json_file(
                        &file,
                        discovered_at,
                        mcp_probe_policy,
                        entities,
                        report_issues,
                        git_cache,
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
    git_cache: &mut GitStateCache,
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
            "command" => build_command_entity(root, &file, discovered_at, git_cache),
            "agent" => build_agent_entity(root, &file, discovered_at, git_cache),
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
    git_cache: &mut GitStateCache,
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
            git_cache,
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
    git_cache: &mut GitStateCache,
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
            git_cache,
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
    git_cache: &mut GitStateCache,
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
            git_cache,
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
    git_cache: &mut GitStateCache,
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
        entities.push(build_skill_entity(
            root,
            &skill_md,
            platform,
            discovered_at,
            git_cache,
        ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{mcp::mcp_probe_result, scan_local_extensions, ScanOptions};
    use std::{
        env, fs,
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
        let mut git_cache = GitStateCache::default();
        scan_plugin_root(
            &root,
            "codex",
            "2026-06-07T00:00:00Z",
            false,
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
        );
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0]["name"], "local-plugin");

        roots.clear();
        entities.clear();
        issues.clear();
        let mut git_cache = GitStateCache::default();
        scan_plugin_root(
            &root,
            "codex",
            "2026-06-07T00:00:00Z",
            true,
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
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
        let mut git_cache = GitStateCache::default();
        scan_configured_root(
            &root,
            "2026-06-07T00:00:00Z",
            false,
            "disabled",
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
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
        let mut git_cache = GitStateCache::default();
        scan_configured_root(
            &root,
            "2026-06-07T00:00:00Z",
            false,
            "disabled",
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
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
        let mut git_cache = GitStateCache::default();

        scan_configured_root(
            &root,
            "2026-06-07T00:00:00Z",
            false,
            "disabled",
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
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
