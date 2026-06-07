use super::common::issue_id;
use serde_json::{json, Value};
use std::path::Path;

pub(crate) fn instruction_health(name: &str, file: &Path, content: &str) -> Value {
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
    issues.extend(encoding_issues(file, content));
    issues.extend(suspicious_content_issues(file, content));

    let status = status_for_issues(&issues);

    json!({
      "status": status,
      "issues": issues,
    })
}

pub(crate) fn plugin_health(name: &str, manifest_path: &Path, manifest: &Value) -> Value {
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
    if let Some(serialized) = serde_json::to_string(manifest).ok() {
        issues.extend(suspicious_content_issues(manifest_path, &serialized));
    }

    let status = status_for_issues(&issues);

    json!({
      "status": status,
      "issues": issues,
    })
}

pub(crate) fn skill_health(
    name: &str,
    skill_md: &Path,
    content: &str,
    description: Option<&str>,
) -> Value {
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
    issues.extend(frontmatter_issues(skill_md, content));
    issues.extend(encoding_issues(skill_md, content));
    issues.extend(broken_local_reference_issues(skill_md, content));
    issues.extend(suspicious_content_issues(skill_md, content));

    let status = status_for_issues(&issues);

    json!({
      "status": status,
      "issues": issues,
    })
}

pub(crate) fn markdown_health(
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
    issues.extend(frontmatter_issues(file, content));
    issues.extend(encoding_issues(file, content));
    issues.extend(broken_local_reference_issues(file, content));
    issues.extend(suspicious_content_issues(file, content));

    let status = status_for_issues(&issues);

    json!({
      "status": status,
      "issues": issues,
    })
}

fn frontmatter_issues(file: &Path, content: &str) -> Vec<Value> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Vec::new();
    }

    let mut issues = Vec::new();
    let mut lines = trimmed.lines();
    lines.next();

    for line in lines.by_ref() {
        let value = line.trim();
        if value == "---" {
            return issues;
        }
        if value.is_empty() || value.starts_with('#') {
            continue;
        }
        if !value.contains(':') {
            issues.push(json!({
              "id": issue_id("frontmatter-format", file),
              "severity": "medium",
              "category": "format",
              "message": "Markdown frontmatter contains a line that does not look like a key-value field.",
              "file": file.to_string_lossy(),
              "evidence": value,
              "recommendation": "Review the YAML frontmatter and keep one key-value field per line.",
            }));
            return issues;
        }
    }

    issues.push(json!({
      "id": issue_id("frontmatter-unclosed", file),
      "severity": "medium",
      "category": "format",
      "message": "Markdown frontmatter starts with --- but has no closing delimiter.",
      "file": file.to_string_lossy(),
      "recommendation": "Close the frontmatter block with a second --- line.",
    }));
    issues
}

fn encoding_issues(file: &Path, content: &str) -> Vec<Value> {
    let patterns = ["\u{fffd}", "Ã", "Â", "â€", "鎵", "鏈", "锛", "�"];
    if patterns.iter().any(|pattern| content.contains(pattern)) {
        return vec![json!({
          "id": issue_id("mojibake", file),
          "severity": "medium",
          "category": "encoding",
          "message": "Content contains characters commonly seen in mojibake.",
          "file": file.to_string_lossy(),
          "recommendation": "Review the file encoding and save the file as UTF-8 if the text is garbled.",
        })];
    }

    Vec::new()
}

fn broken_local_reference_issues(file: &Path, content: &str) -> Vec<Value> {
    let mut issues = Vec::new();
    let parent = file.parent().unwrap_or_else(|| Path::new(""));

    for target in markdown_link_targets(content).into_iter().take(8) {
        if should_skip_reference(&target) {
            continue;
        }

        let without_fragment = target
            .split_once('#')
            .map(|(path, _)| path)
            .unwrap_or(target.as_str());
        let path_part = without_fragment
            .split_once('?')
            .map(|(path, _)| path)
            .unwrap_or(without_fragment)
            .trim();
        if path_part.is_empty() {
            continue;
        }

        let candidate = Path::new(path_part);
        let resolved = if candidate.is_absolute() {
            candidate.to_path_buf()
        } else {
            parent.join(candidate)
        };

        if !resolved.exists() {
            issues.push(json!({
              "id": issue_id("missing-reference", file),
              "severity": "low",
              "category": "path",
              "message": "Markdown references a local file that was not found.",
              "file": file.to_string_lossy(),
              "evidence": path_part,
              "recommendation": "Fix the relative path or remove the stale reference.",
            }));
            break;
        }
    }

    issues
}

fn markdown_link_targets(content: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut rest = content;

    while let Some(start) = rest.find("](") {
        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find(')') else {
            break;
        };
        targets.push(after_start[..end].trim().trim_matches('"').to_string());
        rest = &after_start[end + 1..];
    }

    targets
}

fn should_skip_reference(target: &str) -> bool {
    let lower = target.to_ascii_lowercase();
    target.starts_with('#')
        || lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("mailto:")
        || lower.starts_with("data:")
        || lower.contains("://")
}

fn suspicious_content_issues(file: &Path, content: &str) -> Vec<Value> {
    let patterns = [
        ("shell", "powershell"),
        ("shell", "cmd.exe"),
        ("shell", "bash -c"),
        ("network", "Invoke-WebRequest"),
        ("network", "curl "),
        ("network", "wget "),
        ("eval", "eval("),
        ("eval", "child_process"),
        ("credential", "process.env"),
        ("credential", "Authorization:"),
        ("credential", "api_key"),
        ("credential", "OPENAI_API_KEY"),
    ];
    let lower_content = content.to_ascii_lowercase();
    let mut issues = Vec::new();

    for (kind, pattern) in patterns {
        if lower_content.contains(&pattern.to_ascii_lowercase()) {
            issues.push(json!({
              "id": issue_id(&format!("suspicious-{}", kind), file),
              "severity": "medium",
              "category": "security",
              "message": format!("Suspicious {} handling pattern found: {}.", kind, pattern),
              "file": file.to_string_lossy(),
              "recommendation": "Review this content manually before trusting or executing related commands.",
            }));
            break;
        }
    }

    issues
}

fn status_for_issues(issues: &[Value]) -> &'static str {
    if issues.is_empty() {
        return "ok";
    }

    if issues.iter().any(|issue| {
        issue.get("category").and_then(Value::as_str) == Some("security")
            || issue.get("severity").and_then(Value::as_str) == Some("high")
    }) {
        return "at-risk";
    }

    "needs-review"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn marks_suspicious_skill_content_as_at_risk() {
        let health = skill_health(
            "network-skill",
            Path::new("C:\\skills\\network\\SKILL.md"),
            "# Network Skill\nUse when checking a command.\n\nRun curl https://example.com",
            Some("Use when checking a command."),
        );

        assert_eq!(
            health.get("status").and_then(Value::as_str),
            Some("at-risk")
        );
        assert_eq!(
            health
                .get("issues")
                .and_then(Value::as_array)
                .and_then(|issues| issues.first())
                .and_then(|issue| issue.get("category"))
                .and_then(Value::as_str),
            Some("security")
        );
    }

    #[test]
    fn flags_invalid_frontmatter() {
        let health = skill_health(
            "frontmatter-skill",
            Path::new("C:\\skills\\frontmatter\\SKILL.md"),
            "---\nname frontmatter-skill\n---\nUse when testing frontmatter handling.",
            Some("Use when testing frontmatter handling."),
        );

        assert_has_issue_category(&health, "format");
    }

    #[test]
    fn flags_obvious_mojibake() {
        let health = markdown_health(
            "command",
            "garbled",
            Path::new("C:\\commands\\garbled.md"),
            "# Garbled\n鎵╁睍浣撴",
            Some("Use when testing garbled text."),
        );

        assert_has_issue_category(&health, "encoding");
    }

    #[test]
    fn flags_missing_local_markdown_references() {
        let root = unique_test_dir("missing-reference");
        fs::create_dir_all(&root).expect("create markdown test dir");
        let file = root.join("SKILL.md");
        fs::write(
            &file,
            "# Reference Skill\nUse when checking local references.\n\nSee [missing](docs/missing.md).",
        )
        .expect("write markdown test file");
        let content = fs::read_to_string(&file).expect("read markdown test file");

        let health = skill_health(
            "reference-skill",
            &file,
            &content,
            Some("Use when checking local references."),
        );

        assert_has_issue_category(&health, "path");
        fs::remove_dir_all(root).expect("remove markdown test dir");
    }

    fn assert_has_issue_category(health: &Value, category: &str) {
        let has_category = health
            .get("issues")
            .and_then(Value::as_array)
            .is_some_and(|issues| {
                issues.iter().any(|issue| {
                    issue
                        .get("category")
                        .and_then(Value::as_str)
                        .is_some_and(|value| value == category)
                })
            });
        assert!(has_category, "expected health issue category {category}");
    }

    fn unique_test_dir(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("skilldesk-{name}-{}-{nanos}", std::process::id()))
    }
}

pub(crate) fn mcp_health(
    name: &str,
    config_path: &Path,
    command: Option<&str>,
    url: Option<&str>,
) -> Value {
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

    if url.is_some_and(|value| value.to_ascii_lowercase().starts_with("http://")) {
        issues.push(json!({
          "id": issue_id("mcp-non-https", config_path),
          "severity": "medium",
          "category": "security",
          "message": format!("MCP server '{}' uses a non-HTTPS remote endpoint.", name),
          "file": config_path.to_string_lossy(),
          "recommendation": "Prefer HTTPS for remote MCP endpoints or review why plaintext HTTP is required.",
        }));
    }

    let status = if command.is_none() && url.is_none() {
        "broken"
    } else if issues
        .iter()
        .any(|issue| issue.get("category").and_then(Value::as_str) == Some("security"))
    {
        "at-risk"
    } else {
        "needs-review"
    };

    json!({
      "status": status,
      "issues": issues,
    })
}

#[cfg(test)]
mod mcp_tests {
    use super::*;

    #[test]
    fn flags_non_https_remote_mcp() {
        let health = mcp_health(
            "remote",
            Path::new("C:\\Users\\example\\.mcp.json"),
            None,
            Some("http://example.invalid/mcp"),
        );

        assert_eq!(
            health.get("status").and_then(Value::as_str),
            Some("at-risk")
        );
    }
}
