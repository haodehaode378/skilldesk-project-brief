use super::common::url_host;
use serde_json::{json, Value};
use std::{
    net::{TcpStream, ToSocketAddrs},
    time::{Duration, Instant},
};

pub(crate) fn mcp_probe_result(policy: &str, url: Option<&str>) -> Value {
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
