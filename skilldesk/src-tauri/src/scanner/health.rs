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
    issues.extend(suspicious_content_issues(file, content));

    let status = status_for_issues(&issues);

    json!({
      "status": status,
      "issues": issues,
    })
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
