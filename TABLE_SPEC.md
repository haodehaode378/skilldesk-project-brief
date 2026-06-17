# SkillDesk Table Spec

This document defines table behavior for SkillDesk's desktop workspaces. Tables are core software controls, not decorative report blocks.

Use this with `UI_SPEC.md`, `UX_FLOWS.md`, and `UI_STATES.md` before implementing list, grid, or detail work.

## Table Principles

- Tables must support repeated audit work.
- Tables should remain dense but readable.
- Table interactions must not imply file mutation.
- Long Windows paths must never break layout.
- Selection, sorting, filtering, and detail synchronization should be predictable.

## Shared Table Behavior

### Layout

- Header row remains visually distinct.
- Row height should support Chinese and English without clipping.
- Numeric columns align consistently.
- Status columns use badges with text.
- Empty values use `-`.
- Redacted values use `已脱敏 / Redacted`, not `-`.

### Path Handling

Table cells:

- truncate long paths visually
- prefer middle truncation when implemented
- keep full path available in detail panel
- support tooltip or copy action in future iteration

Detail panels:

- allow full path wrapping
- preserve path separators
- avoid horizontal page overflow

### Selection

Required now:

- click row to select
- selected row stays highlighted
- detail panel updates from selected row

Required next:

- keyboard row navigation
- Enter opens or focuses detail
- Escape clears transient menus or search focus
- selected row remains selected after filters when still present

### Sorting

Required next:

- columns define whether they are sortable
- active sort indicator is visible
- default sort is documented per table
- sorting is stable

Recommended sort cycle:

1. default
2. ascending
3. descending

### Filtering

Required now:

- visible result count
- active filter state
- empty filtered result state

Required next:

- filters persist locally per workspace
- quick reset filters action
- URL-like routing is not required because this is a desktop app

### Search

Search should be case-insensitive.

Search should match normalized visible fields, not raw secrets or full hidden values.

## Extensions Table

Purpose: primary normalized inventory.

Default sort:

1. health status priority: `broken`, `at-risk`, `needs-review`, `ok`
2. issue count descending
3. kind
4. name

Columns:

| Column | Required | Sortable | Filtered by | Notes |
| --- | --- | --- | --- | --- |
| Name | Yes | Yes | Search | Use title when available, otherwise name. |
| Kind | Yes | Yes | Kind filter | Skill, command, agent, plugin, MCP server, instruction file. |
| Platform | Yes | Yes | Platform filter | Codex, Claude Code, shared, unknown. |
| Source | Next | Yes | Source filter | local, system, plugin-cache, marketplace, unknown. |
| Status | Yes | Yes | Status filter | Text badge required. |
| Issues | Yes | Yes | None | Numeric count. |
| Path | Next | No | Search/root filter | Truncate in table, full in detail. |
| Last modified | Next | Yes | None | Local time format. |

Row action:

- Select entity and update detail panel.

Future row secondary actions:

- copy path
- copy safe summary
- reveal in file manager only after explicit product decision

Not allowed:

- run entity
- execute script
- install/update/delete

## Issues Table Or Card List

Purpose: prioritize maintenance work.

Default sort:

1. severity priority: `high`, `medium`, `low`, `info`
2. category
3. entity kind
4. file path

Fields:

| Field | Required | Sortable | Filtered by | Notes |
| --- | --- | --- | --- | --- |
| Severity | Yes | Yes | Severity filter | Badge with text. |
| Category | Yes | Yes | Category filter | metadata, format, encoding, git, security, etc. |
| Entity | Next | Yes | Entity/kind filter | Link/select owning entity. |
| Message | Yes | No | Search | Actionable, modest language. |
| Recommendation | Yes when available | No | Search | Manual review guidance. |
| File | Yes when available | No | Search/root filter | Safe path only. |
| Evidence | Optional | No | Search | No secrets or raw logs. |

Required next behavior:

- selecting an issue can navigate to or reveal the owning entity
- issues can be grouped by entity
- active severity filter is persistent

Not allowed:

- mark arbitrary content as malicious without proof
- auto-fix files
- delete or quarantine files

## MCP Servers Table

Purpose: review configured MCP servers safely.

Default sort:

1. health status priority
2. probe attempted state
3. server name

Columns:

| Column | Required | Sortable | Filtered by | Notes |
| --- | --- | --- | --- | --- |
| Server name | Yes | Yes | Search | From config key or probe info. |
| Platform/source | Next | Yes | Platform/source filter | Codex or Claude config source. |
| Transport | Yes | Yes | Transport filter | stdio, sse, streamable-http, http, unknown. |
| Status | Yes | Yes | Status filter | Text badge. |
| Command/host | Yes | No | Search | Command name or URL host only. |
| Args | Yes for stdio | Yes | None | Count only. |
| Tools | Yes when probed | Yes | None | Count only. |
| Resources | Yes when probed | Yes | None | Count only. |
| Prompts | Yes when probed | Yes | None | Count only. |
| Probe | Yes | Yes | Probe filter | attempted, skipped, failed. |
| Config path | Next | No | Search/root filter | Truncated in table. |

Allowed future actions:

- copy redacted config summary
- run allowed lightweight probe under policy

Not allowed:

- arbitrary MCP tool call buttons
- raw auth headers
- full credential-bearing URLs

## Plugins Table

Purpose: separate plugin inventory from cache noise.

Default sort:

1. cache state: local/system before cache/backup
2. health status priority
3. name

Columns:

| Column | Required | Sortable | Filtered by | Notes |
| --- | --- | --- | --- | --- |
| Name | Yes | Yes | Search | Manifest name or directory name. |
| Source | Yes | Yes | Source filter | local, system, plugin-cache, marketplace, unknown. |
| Version | Yes when available | Yes | Search | Use `-` when missing. |
| Publisher | Yes when available | Yes | Search | Use `-` when missing. |
| Skills | Yes | Yes | None | Count. |
| Commands | Yes | Yes | None | Count. |
| Agents | Yes | Yes | None | Count. |
| MCP servers | Yes | Yes | None | Count. |
| Hooks | Yes | Yes | None | Count, do not execute. |
| Cache status | Yes | Yes | Cache filter | cache, backup, normal. |
| Issues | Yes | Yes | None | Count. |

Grouping requirement:

- local plugins
- bundled/system plugins
- marketplace/cache plugins
- backup/noise groups

## Sources Table

Purpose: explain scanner coverage.

Default sort:

1. status priority: `error`, `missing`, `skipped`, `scanned`
2. kind
3. path

Columns:

| Column | Required | Sortable | Filtered by | Notes |
| --- | --- | --- | --- | --- |
| Root path | Yes | Yes | Search | Full path in detail or tooltip. |
| Kind | Yes | Yes | Kind filter | directory or file. |
| Status | Yes | Yes | Status filter | scanned, missing, skipped, error. |
| Entity count | Next | Yes | None | Count from report when available. |
| Reason | Yes when available | No | Search | Safe reason only. |

Future behavior:

- click root to filter inventory by root
- reset default roots from Settings
- show scanner capability summary above table

## Instruction Files Table Or Cards

Purpose: show project-level instruction files.

Default sort:

1. file type
2. applies-to path

Fields:

| Field | Required | Sortable | Notes |
| --- | --- | --- | --- |
| Title/name | Yes | Yes | File title or filename. |
| File type | Yes | Yes | AGENTS.md, CLAUDE.md, .mcp.json, other. |
| Applies to | Yes | Yes | Path. |
| Lines | Yes | Yes | Numeric. |
| Status | Yes | Yes | Health status. |
| Path | Yes | No | Truncated in table. |

## Settings Tables And Editors

Settings may use table-like editors for scan roots.

Scan root editor requirements:

- one root per line
- preserve Windows examples
- trim empty lines on save
- do not mutate extension files
- show help text directly below editor

Future scan root table:

| Column | Required | Notes |
| --- | --- | --- |
| Enabled | Next | Preference only. |
| Root path | Yes | Editable preference. |
| Kind | Next | Inferred. |
| Last status | Next | From latest report. |
| Reason | Next | Safe summary. |

## Table Acceptance Checklist

- Every table has a documented default sort.
- Every table has documented columns and filters.
- Long Windows paths do not create horizontal overflow.
- Status values are text-visible, not color-only.
- Selection remains stable and updates details.
- Empty filtered results explain the filter state.
- Redacted values are not confused with unknown values.
- No table action executes agents, scripts, hooks, plugin commands, or arbitrary MCP tools.
