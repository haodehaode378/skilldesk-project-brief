# Changelog

All notable changes to SkillDesk are documented in this file.

The format follows Keep a Changelog, and this project uses semantic versioning.

## [Unreleased]

### Added

- Add GitHub Actions CI for frontend typecheck, lint, Vitest, Cargo tests, and Cargo check.
- Ignore local audit and scan output files.

### Fixed

- Report `.agents\skills` as the shared platform instead of Codex.
- Avoid duplicate file metadata reads when building common entity fields.

## [0.1.0] - 2026-06-09

### Added

- First Windows desktop MVP built with Tauri, React, Vite, TypeScript, and Rust.
- Read-only local scanning for Codex skills, `.agents` skills, Claude Code skills, Claude slash commands, Claude agents, plugins, MCP configs, and project instruction files.
- Chinese-first UI with English language switch in Settings.
- Overview, extensions, detail, MCP, plugins, sources, issues, and settings views.
- Configurable scan roots, plugin cache scanning policy, and MCP probe policy.
- JSON report export.
- Windows release installers: NSIS `.exe` and MSI.

### Security

- Default read-only scanning without running agents, scripts, hooks, plugin commands, or arbitrary MCP tools.
- Redaction for credential-bearing Git remotes and MCP URLs.
- Default disabled MCP reachability probing.

[Unreleased]: https://github.com/haodehaode378/skilldesk-project-brief/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/haodehaode378/skilldesk-project-brief/releases/tag/v0.1.0
