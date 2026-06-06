# SkillDesk MVP

## Objective

Build the first read-only SkillDesk version as a Windows-first desktop dashboard for local agent extensions.

The UI defaults to Chinese and supports switching between Chinese and English in Settings.

The MVP should answer:

- What local agent extensions exist?
- Which platform can use them?
- Where did they come from?
- Which ones need review?
- Which files, references, Git state, or MCP servers look broken?

## In Scope

The MVP scans and reports on:

- Codex skills.
- `.agents` skills.
- Claude Code skills.
- Claude Code slash commands.
- Claude Code agents.
- Codex and Claude Code plugin manifests.
- Plugin-bundled skills at summary level.
- MCP server configurations and lightweight health state.
- Project instruction files such as `AGENTS.md`, `CLAUDE.md`, and `.mcp.json`.

## Out Of Scope

The MVP does not:

- Run agents.
- Run scripts.
- Run hooks.
- Run plugin commands.
- Call arbitrary MCP tools.
- Install, update, delete, or rewrite extensions.
- Create git worktrees.
- Open embedded terminals.
- Manage memory or session replay.
- Require cloud accounts.

It also does not require machine translation or AI translation services for core UI text. Core UI strings should be maintained as product copy.

## Default Scan Roots

Use configurable scan roots with these Windows defaults:

```text
%USERPROFILE%\.codex\skills
%USERPROFILE%\.agents\skills
%USERPROFILE%\.claude\skills
%USERPROFILE%\.claude\commands
%USERPROFILE%\.claude\agents
%USERPROFILE%\.codex\config.toml
%USERPROFILE%\.claude\mcp-configs
```

Optional advanced roots:

```text
%USERPROFILE%\.codex\plugins
%USERPROFILE%\.claude\plugins
```

## Default Exclusions

Exclude by default:

- `node_modules`
- `sessions`
- `logs`
- `history.jsonl`
- `auth.json`
- `credentials.json`
- `.sandbox-secrets`
- `.tmp`
- `tmp`
- backup folders
- marketplace expansion noise
- SQLite database files

## Required Scanner Output

The scanner must produce a `ScanReport` containing:

- scan metadata
- scanned roots
- skipped roots
- discovered entities
- health issues
- aggregate counts
- non-sensitive warnings

The scanner must be able to output JSON.

## Required Entity Types

The MVP must normalize at least these entities:

- `skill`
- `command`
- `agent`
- `plugin`
- `mcp-server`
- `instruction-file`

## Required Health Checks

Skill checks:

- `SKILL.md` exists.
- Frontmatter parses.
- `name` is present when required.
- `description` is present and not empty.
- Referenced local files exist.
- Obvious mojibake is detected.
- Suspicious shell, network, eval, or credential patterns are flagged.

Command and agent checks:

- Markdown file exists.
- Name is discoverable.
- Duplicate names are flagged.
- Obvious mojibake is detected.
- Suspicious command patterns are flagged.

Plugin checks:

- Manifest exists where expected.
- Plugin name and version/source are discoverable when present.
- Bundled skill count is summarized.
- Cache and backup noise are identified.

MCP checks:

- Server name is discovered.
- Source config path is recorded.
- Transport type is inferred when possible.
- Lightweight list probes may report tools/resources/prompts availability.
- Unreachable or malformed server configs are flagged.

Git checks:

- Remote URL is detected when available.
- Branch and commit are detected.
- Dirty state is detected.
- Ahead/behind may be local-only unless an explicit refresh action is added later.

## UI Requirements

The first desktop UI should include:

- Overview dashboard.
- Extension list with search and filters.
- Detail view for a selected entity.
- MCP servers view.
- Plugins view.
- Sources view.
- Issues view.
- Settings view.

Settings must include:

- language switch: Chinese / English
- scan root preferences
- plugin cache scan mode
- MCP probe policy

Overview should show:

- total extensions
- skills
- commands
- agents
- plugins
- MCP servers
- needs review
- broken
- possible mojibake
- dirty Git sources

## Success Criteria

The MVP is acceptable when:

- It scans the default roots without executing untrusted content.
- It reports skills, commands, agents, plugins, MCP servers, and instruction files in one normalized model.
- It shows concrete health issues with file paths and recommendations.
- It handles missing roots and permission errors without crashing.
- It does not display secrets or raw auth/log/session content.
- It can export or print a JSON scan report.
- It runs as a local Tauri desktop app.
- It starts in Chinese by default and can switch to English from Settings.

## Verification Criteria

Before calling MVP complete:

- Unit tests cover path normalization, frontmatter parsing, issue generation, and report aggregation.
- A Windows local scan completes on the known default roots.
- Mojibake detection catches known bad samples.
- MCP probe tests use a fixture or controlled local server.
- UI renders non-empty dashboard, list, and detail views from fixture data.
- Bilingual UI string coverage exists for primary navigation, dashboard labels, status labels, and Settings.
