# SkillDesk Software UX Spec

SkillDesk is a Windows-first desktop software product for auditing the local agent-extension ecosystem. It is not a website, not a landing page, and not an agent runner.

The interface is Chinese-first. The default locale is `zh-CN`; English is available from Settings.

## UX Positioning

SkillDesk should feel like a local developer operations console:

- dense enough for repeated maintenance work
- calm enough for scanning and comparison
- explicit about safety boundaries
- optimized for long Windows paths and local filesystem metadata
- clear that every MVP action is read-only

The first screen is the application dashboard, not a marketing hero.

Avoid:

- landing-page hero sections
- decorative product storytelling
- mascot-driven visuals
- large empty promotional cards
- agent execution controls
- terminal-like command runners

Prefer:

- compact tables
- split list/detail workspaces
- status badges with text
- issue-first detail panels
- visible scan provenance
- stable layouts for long names and paths

## Software Frame

SkillDesk runs inside a Tauri desktop window.

Recommended initial window:

- default width: 1180-1280 px
- default height: 760-860 px
- minimum width: 960 px for full desktop layout
- minimum height: 640 px

The app should remain usable when narrower than 960 px by stacking panels, but the primary target is a desktop window, not a mobile webpage.

The UI must not assume browser chrome, external navigation, or responsive marketing-page behavior.

## App Shell

The app shell has three persistent regions:

- Sidebar navigation.
- Top command/status bar.
- Main work area.

### Sidebar

Purpose: stable orientation across the software.

Required items:

- 概览 / Overview
- 扩展清单 / Extensions
- MCP 服务 / MCP Servers
- 插件 / Plugins
- 来源 / Sources
- 问题队列 / Issues
- 设置 / Settings

Sidebar rules:

- The active item is visually persistent.
- The sidebar includes a small read-only safety note.
- Navigation labels remain text-first; icons are optional.
- Do not add runner concepts such as Tasks, Sessions, Worktrees, Terminal, or Memory.

### Top Command Bar

Purpose: expose current scan context and safe commands.

Required content:

- product/workspace title
- current report source: 示例数据, 缓存报告, 本机扫描, 扫描失败
- language switch or current locale action
- scan local roots button
- export JSON button
- export Markdown button

Rules:

- The scan button must communicate read-only behavior through nearby UI copy or status context.
- Export buttons are disabled while scanning.
- Scan progress should not block navigation unless data consistency requires it.
- Error messages appear below the top bar and wrap long paths/messages.

## Core User Journeys

### First Launch

Goal: understand what SkillDesk does without onboarding screens.

Expected behavior:

1. Show fixture or empty local report.
2. Display clear report source.
3. Show a primary action to scan local roots.
4. Show the read-only safety note.
5. Avoid any setup wizard for the MVP.

Success state:

- The user can tell SkillDesk audits local extensions.
- The user can tell it will not execute scripts, hooks, plugin commands, or agents.

### Run Local Scan

Goal: refresh local inventory safely.

States:

- idle
- scanning
- completed
- failed with previous report still visible

Behavior:

- While scanning, show a scanning state in the top bar and disable scan/export commands that conflict.
- Keep existing report visible during scan unless no prior report exists.
- On completion, update totals, entities, roots, issues, and report source.
- On failure, preserve the previous report and show a concise error.

The scan flow must never imply that SkillDesk is running agent code.

### Triage Issues

Goal: decide what needs review first.

Expected path:

1. Start from Overview or Issues.
2. Filter by severity/category/status.
3. Select an issue or entity.
4. Read file path, evidence, and recommendation.
5. Decide what to inspect manually outside SkillDesk.

MVP behavior:

- SkillDesk reports recommendations.
- SkillDesk does not fix, overwrite, delete, install, or update files.

### Inspect Extension

Goal: understand one local extension or config object.

Expected path:

1. Open Extensions.
2. Search or filter.
3. Select a row.
4. Read detail panel.
5. Review related issues and metadata.

The list/detail layout is the primary desktop work pattern.

### Configure Preferences

Goal: adjust read-only scan preferences.

Settings should cover:

- UI language
- scan roots
- plugin cache scanning mode
- MCP probe policy
- cached report clearing

Settings should not include:

- agent execution
- script execution
- plugin installation
- auto-repair
- auto-update

## Work Areas

## Overview

Purpose: summarize local ecosystem health and point users to the next review target.

Required metrics:

- 扩展总数 / Total extensions
- Skills
- Commands
- Agents
- Plugins
- MCP servers
- Instruction files
- Needs review
- At risk
- Broken
- Possible mojibake
- Dirty Git sources

Required sections:

- health summary
- current report summary
- preview of extension inventory
- preview of issue queue

Design rules:

- Use a compact grid, not a hero section.
- Keep totals scannable at a glance.
- Use status text plus color; never color alone.
- Show report source and generation time near the metrics.

## Extensions

Purpose: searchable inventory of all normalized entities.

Required filters:

- kind
- platform
- source
- health status
- Git state
- root path

Required columns:

- name
- kind
- platform
- source
- status
- issues
- path
- last modified

Rules:

- Row selection opens the detail panel.
- Long paths truncate visually but remain accessible in detail.
- Search should match name, title, kind, platform, source, and path.
- Empty filtered results use a concise empty state.

## Detail Panel

Purpose: explain one entity enough to decide what to inspect next.

Required sections:

- summary
- health issues
- source and paths
- parsed metadata
- related files
- Git state
- recommendations

Entity-specific detail:

- Skills: `SKILL.md` path, frontmatter summary, script/reference/asset counts.
- Commands: file path, discovered name, namespace or command metadata.
- Agents: file path, declared model, declared tools.
- Plugins: manifest path, version, publisher/source, bundled counts, cache/backup status.
- MCP servers: config path, transport, command or host summary, probe status, safe counts, latest safe error.
- Instruction files: file type, applies-to path, line count.

Rules:

- The detail panel must not show raw secrets, auth headers, or session/log contents.
- If data is unavailable, show `-` or a localized empty value.
- Recommendations should be modest and actionable.

## MCP Servers

Purpose: show configured MCP servers and whether they look usable.

Required columns or card fields:

- server name
- platform/config source
- transport
- status
- tools count
- resources count
- prompts count
- probe state
- warning count

Allowed MVP actions:

- copy redacted config summary
- run lightweight probe only when policy allows it

Forbidden actions:

- arbitrary MCP tool calls
- raw auth header display
- full secret-bearing URL display
- server command execution beyond explicitly allowed lightweight probe behavior

## Plugins

Purpose: separate useful plugin inventory from cache and marketplace noise.

Required groupings:

- local plugins
- bundled/system plugins
- marketplace or cache plugins
- backup/noise groups

Required fields:

- name
- source
- version
- publisher
- skills count
- commands count
- agents count
- MCP servers count
- hooks count
- cache status
- issues

Rules:

- Plugin caches are summary-level by default.
- The UI should make cache/noise status visible without making it feel like primary user content.

## Sources

Purpose: make scan roots and scanner capability explicit.

Required fields:

- root path
- root type
- enabled or configured state
- last scan status
- skipped or error reason
- entity count when available

Required controls:

- edit scan roots
- reset defaults
- include plugin cache toggle or mode
- MCP probe policy selector

MVP constraint:

- Any future write action must ask for explicit confirmation, create a backup where applicable, and be reversible.
- Current MVP source handling should remain read-only preference storage.

## Issues

Purpose: prioritize maintenance work.

Required filters:

- severity
- category
- entity kind
- platform
- source root

Required fields:

- severity
- category
- entity
- message
- file
- evidence when safe
- recommendation

Rules:

- Do not claim arbitrary content is malicious.
- Use `at-risk` or security-pattern language for suspicious content.
- Keep issue messages concrete and review-oriented.

## Settings

Purpose: keep product-wide preferences explicit.

Required controls:

- language: Chinese / English
- scan roots
- plugin cache scan mode
- MCP probe policy
- clear cached report
- report export preference when added

Language behavior:

- Chinese is the default.
- English is available from Settings and may also be reachable from the top bar.
- Language choice is stored locally.
- Scanner data remains language-neutral.
- UI labels and stable issue-code messages are localized at presentation time where practical.

## State Model

### Report Source States

Use these visible states:

- 示例数据 / Fixture data
- 缓存报告 / Cached report
- 正在扫描 / Scanning
- 本机扫描 / Local scan
- 扫描失败 / Scan failed

Each state should answer:

- what data is currently displayed
- when it was generated
- whether scan/export actions are available

### Health States

Use status categories, not numeric scores:

- `ok`
- `needs-review`
- `at-risk`
- `broken`

Status display rules:

- status badges include text
- color is semantic and consistent
- icons are optional but must not replace text
- do not use red for anything except `broken`, high severity, or hard errors

### Empty States

Empty states should be concise and local to the affected panel.

Examples:

- 没有发现扩展。
- 当前筛选条件没有匹配项。
- 这个扫描来源不存在。
- MCP 探测已禁用。
- 插件缓存当前仅汇总 manifest。

Empty states must not become product tours.

### Error States

Error messages should:

- preserve the previous usable report when possible
- show safe file paths and safe metadata
- avoid secrets, raw logs, auth files, or session contents
- include a next step when clear

Examples:

- scan root missing
- permission denied
- invalid JSON/TOML/Markdown frontmatter
- MCP probe failed
- report export failed

## Desktop Interaction Rules

Keyboard:

- Tab order follows sidebar, top actions, main content.
- Focus rings are visible.
- Tables should remain readable without hover-only information.

Mouse:

- Row hover helps scanning but selected row must be persistent.
- Buttons use familiar text labels; icons are optional.
- Destructive or future write actions require confirmation.

Path handling:

- Windows paths must not break layout.
- Use truncation in tables and full wrapping in detail/errors.
- Prefer middle truncation when implemented.

Performance:

- Large inventories should keep navigation responsive.
- Filters should not reflow the full shell.
- Avoid decorative animation in the MVP.

## Visual System

Use restrained developer-tool styling.

Recommended palette:

- background: light neutral gray
- sidebar: dark neutral
- primary accent: muted green/teal
- `ok`: green
- `needs-review`: amber
- `at-risk`: orange
- `broken`: red
- neutral/unknown: gray

Typography:

- Use system UI fonts suitable for Chinese and English.
- Do not use oversized marketing display type.
- Reserve large numbers for metrics only.
- Keep table text compact and readable.

Component rules:

- Border radius should stay modest, around 6-8 px.
- Avoid cards inside cards.
- Use panels for work areas, not decorative floating sections.
- Keep controls predictable and desktop-like.

Components must handle:

- Chinese and English labels
- long Windows paths
- long skill names
- missing metadata
- empty roots
- permission errors
- large inventories

## OpenPencil Use

OpenPencil is a design aid, not a runtime dependency.

OpenPencil should be used for:

- overview dashboard layout
- extension list/detail split view
- MCP servers view
- settings layout
- scan/error state exploration

OpenPencil output should inform the React UI, but the implementation remains hand-built in the Tauri + React app.

## Acceptance Checklist

Before treating the software UX as MVP-ready:

- The first screen is the functional dashboard.
- The user can identify current report source.
- The user can run a read-only scan.
- The user can switch Chinese/English UI.
- The user can inspect an entity from list to detail.
- The user can triage issues by severity and category.
- Long Windows paths do not break layout.
- Missing roots and permission errors have readable states.
- The UI never suggests it can run agents.
- The UI never exposes secrets, raw auth files, session logs, or raw credential values.
- Desktop and narrow-window layouts have no horizontal overflow.
- Chinese copy passes mojibake checks.
