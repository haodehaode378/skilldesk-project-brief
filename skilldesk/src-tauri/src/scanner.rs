mod common;
mod entities;
mod git;
mod health;
mod mcp;
mod roots;

use common::*;
use git::GitStateCache;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::{env, path::PathBuf};

const SCHEMA_VERSION: &str = "0.1";

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
    let mut git_cache = GitStateCache::default();

    if let Some(home) = home_dir.as_ref() {
        roots::scan_configured_roots(
            home,
            options.scan_roots.as_deref(),
            &now,
            options.include_plugin_caches,
            mcp_probe_policy,
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
        );
        roots::scan_instruction_files(
            home,
            &now,
            mcp_probe_policy,
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
        );
    }

    if let Ok(current_dir) = env::current_dir() {
        roots::scan_instruction_files(
            &current_dir,
            &now,
            mcp_probe_policy,
            &mut roots,
            &mut entities,
            &mut issues,
            &mut git_cache,
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
