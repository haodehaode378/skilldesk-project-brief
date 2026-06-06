# SkillDesk Roadmap

SkillDesk should progress in small, verifiable phases. Each phase should produce a working artifact and avoid expanding into agent execution.

## Phase 0: Scanner Prototype

Goal: prove the read-only scanner model before building the desktop shell.

Deliverables:

- TypeScript scanner core.
- Configurable scan roots.
- Default Windows roots.
- Normalized `ScanReport` JSON output.
- Fixture-based tests for parsing and issue generation.

Must cover:

- Codex skills.
- `.agents` skills.
- Claude Code skills.
- Claude commands.
- Claude agents.
- MCP config discovery.
- Instruction files.

May cover:

- Summary-level plugin manifest counts.
- Summary-level plugin-bundled skill counts.

Must not:

- Run scripts.
- Run hooks.
- Call arbitrary MCP tools.
- Mutate files.

Exit criteria:

- `skilldesk scan --json` can produce a non-empty report from fixture roots.
- Local Windows default-root scan handles missing paths and permission errors.
- Unit tests cover path normalization, frontmatter parsing, and health issue aggregation.

## Phase 1: Desktop MVP

Goal: ship a local Tauri desktop app that displays scanner results.

Deliverables:

- Tauri app shell.
- React + Vite + TypeScript frontend.
- Overview dashboard.
- Extension list.
- Detail view.
- Issues view.
- Scan trigger and scan result loading.

Must show:

- total entities
- skills
- commands
- agents
- plugins
- MCP servers
- instruction files
- broken entities
- needs-review entities
- possible mojibake

Exit criteria:

- Desktop app opens on Windows.
- Dashboard renders fixture data and real scan data.
- Detail view shows entity metadata and health issues.
- No untrusted local extension code is executed.

## Phase 2: MCP And Plugin Depth

Goal: improve MCP and plugin visibility without changing the read-only safety model.

Deliverables:

- MCP server detail view.
- Optional lightweight MCP probes.
- Plugin manifest detail view.
- Cache noise classification.
- Duplicate and stale-cache warnings.

MCP probe policy:

- Local MCP probes may be enabled by default only for safe list calls.
- Remote MCP probes should require explicit user opt-in.
- Never call arbitrary tools in this phase.

Exit criteria:

- MCP servers show config source, transport, reachability, and tool/resource/prompt counts when safely available.
- Plugin cache entries are separated from user-authored local extensions.
- Cache and marketplace noise do not inflate primary inventory counts.

## Phase 3: Reports And History

Goal: make SkillDesk useful for repeated maintenance.

Deliverables:

- JSON report export.
- Markdown report export.
- Last scan cache.
- Diff from previous scan.
- Basic trend/history view.

Storage:

- Start with JSON cache.
- Add SQLite only if history and filtering require it.

Exit criteria:

- User can export a report without exposing secrets.
- User can see new, removed, changed, and newly broken entities between scans.

## Phase 4: Controlled Write Actions

Goal: add optional maintenance actions after the read-only product is stable.

Potential actions:

- Refresh Git metadata.
- Generate suggested patches.
- Export recommended cleanup plans.
- Generate or refresh companion metadata after confirmation.

Required safeguards:

- Explicit confirmation.
- Backup before write.
- Preview diff.
- Reversible operation.
- No silent overwrite.

Still out of scope:

- Running agents.
- Running arbitrary extension scripts.
- Worktree orchestration.
- Embedded terminal task execution.

## Future Ideas

Possible future work:

- OpenPencil-assisted UI prototype import.
- SQLite-backed history.
- CI mode for project-level audits.
- Duplicate skill merge suggestions.
- Cross-platform support beyond Windows.

These should not block the MVP.
