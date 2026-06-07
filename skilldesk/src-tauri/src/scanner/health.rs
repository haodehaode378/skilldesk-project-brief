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
