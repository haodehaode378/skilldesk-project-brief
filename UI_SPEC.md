# SkillDesk UI Spec

SkillDesk should feel like a local developer operations tool: dense, calm, fast to scan, and focused on maintenance decisions.

## Product UI Principle

The first screen is the dashboard, not a landing page.

Avoid:

- marketing hero sections
- decorative gradients
- large empty cards
- mascot-driven visuals
- agent-runner controls

Prefer:

- compact tables
- clear status badges
- source filters
- issue-first detail views
- stable layouts that work with long Windows paths

## App Shell

Primary navigation:

- Overview
- Extensions
- MCP Servers
- Plugins
- Sources
- Issues
- Settings

Top bar:

- app name
- last scan time
- scan button
- export button
- settings button

The scan button must communicate that the scan is read-only.

## Overview

Purpose: summarize local ecosystem health.

Required metrics:

- Total extensions
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

- Status summary.
- Top issues.
- Recent scan roots.
- MCP health summary.
- Plugin cache noise summary.

## Extensions View

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

Long paths should truncate in the middle and show the full path on hover or detail.

## Detail View

Purpose: explain one entity enough to decide what to do next.

Required sections:

- summary
- health issues
- source and paths
- parsed metadata
- related files
- Git state
- recommendations

For skills, show:

- `SKILL.md` path
- frontmatter summary
- scripts count
- references count
- assets count

For commands and agents, show:

- file path
- discovered name
- declared tools or command metadata when present

For plugins, show:

- manifest path
- version/source when present
- bundled counts
- whether it came from cache, backup, marketplace, or local source

For MCP servers, show:

- config path
- transport
- host or command summary
- probe status
- tools/resources/prompts counts when available
- latest error when safe to show

## MCP Servers View

Purpose: show configured MCP servers and whether they look usable.

Required columns:

- server name
- platform/config source
- transport
- status
- tools
- resources
- prompts
- last probe
- warning count

MCP actions:

- Probe local server.
- Copy redacted config summary.

Do not include:

- arbitrary tool call buttons
- raw auth headers
- full secret-bearing URLs

## Plugins View

Purpose: separate user-authored extensions from cache and marketplace noise.

Required sections:

- local plugins
- system or bundled plugins
- marketplace/cache plugins
- backup/noise groups

Required columns:

- name
- source
- version
- skills
- agents
- MCP servers
- cache status
- issues

## Sources View

Purpose: make scan roots explicit and configurable.

Required fields:

- root path
- type
- enabled
- last status
- entity count
- skipped reason

Required controls:

- add root
- disable root
- reset defaults
- include plugin cache toggle
- MCP probe policy selector

The include plugin cache toggle should be off or summary-only by default.

## Issues View

Purpose: prioritize maintenance work.

Required filters:

- severity
- category
- entity kind
- platform
- source root

Required columns:

- severity
- category
- entity
- message
- file
- recommendation

Issue messages should be actionable and modest. They should not label files as malicious without proof.

## Visual System

Use restrained developer-tool styling.

Recommended status colors:

- `ok`: green
- `needs-review`: amber
- `at-risk`: orange
- `broken`: red
- neutral/unknown: gray

Do not rely on color alone. Pair color with text and icons.

Components should handle:

- long Windows paths
- long skill names
- missing metadata
- empty roots
- permission errors
- large inventories

## Empty States

Empty states should be concise.

Examples:

- No extensions found.
- This scan root is missing.
- MCP probes are disabled.
- Plugin cache scanning is summary-only.

Do not use empty states to explain the whole product.

## OpenPencil Use

OpenPencil may be used to prototype:

- Overview dashboard.
- Extensions table and detail split view.
- MCP servers view.

OpenPencil output is a design aid only. It should not become a runtime dependency.
