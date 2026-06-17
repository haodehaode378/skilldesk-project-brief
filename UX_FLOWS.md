# SkillDesk UX Flows

This document defines the software-level user flows for SkillDesk. It complements `UI_SPEC.md` by describing task paths, state changes, and acceptance checks.

SkillDesk remains a read-only desktop audit tool. These flows must not introduce agent running, script execution, plugin command execution, arbitrary MCP tool calls, worktree management, or embedded terminals.

## Flow Principles

- Keep the current report visible whenever possible.
- Make the report source explicit before asking the user to trust results.
- Prefer review and inspection over action.
- Treat local extension content as untrusted input.
- Do not expose secrets, auth files, session logs, raw credentials, or secret-bearing URLs.

## F001: First Launch

Goal: help the user understand what SkillDesk audits and what it will not do.

Entry condition:

- No local scan has been completed in this app profile, or no cache is available.

Primary path:

1. Open the SkillDesk desktop window.
2. Show the Overview workspace.
3. Display report source as `示例数据 / Fixture data` or an explicit empty state.
4. Show the read-only safety note in the sidebar.
5. Show `扫描本地来源 / Scan local roots` as the primary action.

Success criteria:

- The user can identify SkillDesk as a local extension audit tool.
- The user can identify that scans are read-only.
- The user can start a local scan without visiting Settings.

Failure and edge states:

- If fixture data is unavailable, show an empty dashboard with the scan action.
- If cached settings are invalid, fall back to defaults and show a non-blocking warning.

## F002: Run Local Scan

Goal: refresh the inventory from configured local roots.

Entry condition:

- The user clicks `扫描本地来源 / Scan local roots`.

Primary path:

1. Set report source state to `正在扫描 / Scanning`.
2. Disable scan and export actions while scan is active.
3. Keep the previous report visible unless no prior report exists.
4. Read configured roots using the read-only scanner.
5. Parse and validate the resulting `ScanReport`.
6. Cache the report locally.
7. Set report source state to `本机扫描 / Local scan`.
8. Update Overview, Extensions, MCP, Plugins, Sources, Issues, and Settings where applicable.

Success criteria:

- The user sees current local totals.
- The report source changes from fixture/cache to local scan.
- No untrusted scripts, hooks, plugin commands, or agents run.

Failure path:

1. Preserve the previous usable report.
2. Set report source state to `扫描失败 / Scan failed`.
3. Show a concise error near the top command bar.
4. Keep scan retry available.

Acceptance checks:

- A missing root does not fail the whole scan.
- A permission error is reported as a root or issue state.
- The UI does not clear the previous report on failure.

## F003: Inspect Extension From Inventory

Goal: inspect one normalized entity and decide what to review next.

Entry condition:

- The user opens `扩展清单 / Extensions`.

Primary path:

1. Show the inventory table.
2. Allow search by name, title, kind, platform, source, and path.
3. Allow filtering by health status and entity kind.
4. Select the first entity by default when results exist.
5. On row selection, update the detail panel.
6. Show entity-specific metadata and health issues.

Success criteria:

- The selected row remains visible and visually persistent.
- The detail panel matches the selected entity.
- Long paths do not break the layout.

Future interaction requirements:

- Keyboard row selection.
- Sortable columns.
- Copy path from detail.
- Filter persistence across app restarts.

## F004: Triage Issue Queue

Goal: prioritize maintenance work without editing local files.

Entry condition:

- The user opens `问题队列 / Issues`.

Primary path:

1. Show all report-level issues.
2. Provide severity filters.
3. Provide search across category, message, recommendation, evidence, and file.
4. Show issue cards with severity, category, message, recommendation, and safe path/evidence.

Success criteria:

- The user can isolate high-priority issues quickly.
- The user can understand the recommended manual review.
- The UI does not claim content is malicious without proof.

Future interaction requirements:

- Click issue to open the owning entity detail.
- Group issues by entity, severity, and category.
- Show safe issue codes for localization and documentation.

## F005: Review MCP Servers

Goal: inspect configured MCP servers without invoking arbitrary tools.

Entry condition:

- The user opens `MCP 服务 / MCP Servers`.

Primary path:

1. Show MCP server cards or table rows.
2. Display server name, config source, transport, status, safe command or host summary, probe state, and safe counts.
3. Hide or redact secret-bearing arguments and URLs.
4. Respect the configured MCP probe policy.

Success criteria:

- The user can tell which servers need review.
- No arbitrary MCP tools are callable from the UI.
- Secret-bearing config values are not displayed.

Future interaction requirements:

- Copy redacted config summary.
- Run allowed lightweight local probe with explicit policy.
- Explain disabled probe state clearly.

## F006: Review Plugins

Goal: separate useful plugin inventory from cache and marketplace noise.

Entry condition:

- The user opens `插件 / Plugins`.

Primary path:

1. Show plugin manifests discovered in scan results.
2. Display source, version, publisher, bundled counts, cache state, backup state, and issues.
3. Keep plugin cache entries summary-level by default.

Success criteria:

- The user can distinguish local plugins from cache or marketplace entries.
- Cache entries do not dominate the primary inventory.
- The UI avoids implying plugin commands were executed.

Future interaction requirements:

- Group by local/system/cache/backup.
- Show cache family and duplicate groups.
- Provide summary-only mode explanation.

## F007: Review Sources

Goal: understand what was scanned, skipped, missing, or errored.

Entry condition:

- The user opens `来源 / Sources`.

Primary path:

1. Show scanner self-check counts.
2. Show capability policy: read-only scanning, plugin cache mode, MCP probe policy.
3. Show root list with path, kind, status, and reason.
4. Show discovered project instruction files.

Success criteria:

- The user can identify missing or skipped roots.
- The user can understand why a root was skipped or errored.
- Root paths remain readable.

Future interaction requirements:

- Jump from root to filtered inventory.
- Show entity count per root.
- Support reset-to-default scan roots from Settings.

## F008: Configure Settings

Goal: adjust app preferences without changing extension files.

Entry condition:

- The user opens `设置 / Settings`.

Primary path:

1. Show current UI locale.
2. Allow switching Chinese and English.
3. Show plugin cache scan mode.
4. Show MCP probe policy selector.
5. Show editable scan roots.
6. Show scan safety text.
7. Allow clearing cached report.

Success criteria:

- Language changes apply immediately.
- Settings are saved locally.
- Scanner output data remains language-neutral.

Write-safety rule:

- Settings may update SkillDesk local preferences.
- Settings must not modify third-party extension files in the MVP.

## F009: Export Report

Goal: save a local copy of the current report.

Entry condition:

- The user clicks JSON or Markdown export.

Primary path:

1. Validate the current report payload.
2. Write the export into the configured downloads location.
3. Show a success message with the local export path.

Failure path:

1. Preserve the current report.
2. Show export failure message with a safe error.

Success criteria:

- The exported report exists locally.
- The UI shows a readable confirmation.
- The export does not include raw secrets, auth files, session logs, or raw credential values.

Future interaction requirements:

- Offer redacted export mode.
- Explain that paths and metadata may be included.

## Flow Test Matrix

| Flow | Fixture data | Cached report | Local scan | Error state | Narrow window |
| --- | --- | --- | --- | --- | --- |
| First launch | Required | Optional | Not required | Required | Required |
| Run local scan | Required | Required | Required | Required | Required |
| Inspect extension | Required | Required | Required | Optional | Required |
| Triage issues | Required | Required | Required | Optional | Required |
| MCP review | Required | Required | Required | Required | Required |
| Settings | Required | Required | Optional | Optional | Required |
| Export report | Required | Required | Required | Required | Optional |
