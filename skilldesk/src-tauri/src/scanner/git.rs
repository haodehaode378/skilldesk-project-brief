use serde_json::{json, Map, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn attach_git_state(entity: &mut Map<String, Value>, path: &Path) {
    let Some(git_root) = find_git_root(path) else {
        return;
    };

    let Some(git_state) = read_git_state(&git_root) else {
        return;
    };

    entity.insert("git".to_string(), git_state);
}

fn find_git_root(path: &Path) -> Option<PathBuf> {
    let mut current = if path.is_file() {
        path.parent()
    } else {
        Some(path)
    };

    while let Some(dir) = current {
        let git_dir = dir.join(".git");
        if git_dir.is_dir() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }

    None
}

fn read_git_state(root: &Path) -> Option<Value> {
    let git_dir = root.join(".git");
    let head = fs::read_to_string(git_dir.join("HEAD")).ok()?;
    let head = head.trim();
    let mut state = Map::new();
    state.insert("root".to_string(), json!(root.to_string_lossy()));
    state.insert("dirty".to_string(), json!(false));

    if let Some(ref_name) = head.strip_prefix("ref: ") {
        state.insert(
            "branch".to_string(),
            json!(ref_name.trim_start_matches("refs/heads/")),
        );
        if let Ok(commit) = fs::read_to_string(git_dir.join(ref_name)) {
            state.insert("commit".to_string(), json!(commit.trim()));
        }
        state.insert("detached".to_string(), json!(false));
    } else if !head.is_empty() {
        state.insert("commit".to_string(), json!(head));
        state.insert("detached".to_string(), json!(true));
    }

    if let Some(remote_url) = read_origin_remote_url(&git_dir.join("config")) {
        state.insert("remoteUrl".to_string(), json!(remote_url));
    }

    Some(Value::Object(state))
}

fn read_origin_remote_url(config_path: &Path) -> Option<String> {
    let config = fs::read_to_string(config_path).ok()?;
    let mut in_origin = false;

    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_origin = trimmed == r#"[remote "origin"]"#;
            continue;
        }
        if in_origin {
            if let Some(url) = trimmed.strip_prefix("url =") {
                return Some(sanitize_remote_url(url.trim()));
            }
        }
    }

    None
}

fn sanitize_remote_url(url: &str) -> String {
    let Some((scheme, rest)) = url.split_once("://") else {
        return url.to_string();
    };
    let Some((_, host_and_path)) = rest.split_once('@') else {
        return url.to_string();
    };
    format!("{}://{}", scheme, host_and_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn reads_branch_commit_and_sanitized_remote() {
        let root = env::temp_dir().join(format!(
            "skilldesk-git-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let git_dir = root.join(".git");
        let refs_dir = git_dir.join("refs").join("heads");
        fs::create_dir_all(&refs_dir).unwrap();
        fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n").unwrap();
        fs::write(refs_dir.join("main"), "abc123\n").unwrap();
        fs::write(
            git_dir.join("config"),
            "[remote \"origin\"]\n  url = https://token@example.com/repo.git\n",
        )
        .unwrap();

        let state = read_git_state(&root).unwrap();

        assert_eq!(state["branch"], "main");
        assert_eq!(state["commit"], "abc123");
        assert_eq!(state["remoteUrl"], "https://example.com/repo.git");

        fs::remove_dir_all(root).unwrap();
    }
}
