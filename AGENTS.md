# SkillDesk Agent Instructions

## Project Role

SkillDesk is a Windows-first local health dashboard for agent extensions:

- skills
- slash commands
- agents
- plugins
- MCP servers
- project instruction files such as `AGENTS.md`, `CLAUDE.md`, and `.mcp.json`

SkillDesk is not an agent runner.

SkillDesk is a Chinese-first bilingual product. The default UI language is Chinese. Users should be able to switch between Chinese and English in Settings.

```text
Elves runs agents.
SkillDesk audits the local ecosystem agents depend on.
```

## Current Status

This repository is currently a project brief, not an implementation project.

- Keep `skilldesk-project-brief.md` as the product baseline.
- Use this `AGENTS.md` as the execution baseline for future agents.
- Do not scaffold the app until explicitly asked.
- Planning documents may be committed and pushed to GitHub after user confirmation.

## Git And Push Policy

Canonical planning repository:

```text
https://github.com/haodehaode378/skilldesk-project-brief.git
```

Implementation location:

```text
E:\ai-skilldesk-project-brief\<implementation-folder>
```

The implementation should live in a new subfolder under this planning folder unless the user changes the location.

When implementation begins and the user asks to use Git:

- Initialize or connect the repository only after explicit confirmation.
- Use conventional commit prefixes such as `feat:`, `fix:`, `docs:`, `test:`, and `chore:`.
- After each completed implementation step, commit and push the completed step.
- Planning baseline documents are allowed in the planning repository after the user confirms.
- Do not push private notes, local scan outputs, auth files, session logs, or machine-specific credentials.
- Never include secrets, local auth files, session logs, or machine-specific credentials in commits.

## Product Direction

The selected implementation stack is:

```text
Tauri + React + Vite + TypeScript
```

Architecture baseline:

- Desktop shell: Tauri.
- Frontend: React, Vite, TypeScript.
- Scanner core: TypeScript package with CLI-first boundaries.
- Cache: JSON first, SQLite later.
- Tests: Vitest for scanner and model logic.
- Package manager: pnpm.
- Default UI language: Chinese.
- Supported UI languages: Chinese and English.
- UI prototyping: OpenPencil.
- Image generation: optional prompt generation only; not an MVP dependency.

## MVP Boundary

The MVP is read-only.

It should scan and report on:

- Codex skills.
- Claude Code skills.
- Claude Code slash commands.
- Claude Code agents.
- Codex and Claude Code plugin manifests.
- Plugin-bundled skills at summary level.
- MCP server configurations and lightweight health status.
- Project instruction files.

The MVP should default to Chinese UI copy and include a Settings language switch for Chinese and English.

It must not:

- Run skill scripts.
- Run hooks.
- Run plugin commands.
- Run or orchestrate agents.
- Create worktrees.
- Open embedded terminals.
- Manage memory or session replay.
- Auto-update, delete, or overwrite user files.

## Default Local Scan Scope

Default scan roots should include:

```text
%USERPROFILE%\.codex\skills
%USERPROFILE%\.agents\skills
%USERPROFILE%\.claude\skills
%USERPROFILE%\.claude\commands
%USERPROFILE%\.claude\agents
%USERPROFILE%\.codex\config.toml
%USERPROFILE%\.claude\mcp-configs
```

Plugin caches are optional advanced scan sources. By default, only summarize obvious plugin manifests and bundled skill counts.

Default exclusions:

- sessions
- history files
- logs
- sqlite databases
- credentials
- auth files
- sandbox secrets
- temporary directories
- backups
- node_modules
- marketplace cache expansion noise

## Safety Rules

All scan and audit work must treat local extensions as untrusted content.

- Do not execute third-party scripts, hooks, commands, or binaries during scanning.
- Do not print secrets, tokens, API keys, auth headers, session contents, or raw logs.
- Report paths, counts, object types, file sizes, mtimes, hashes, and non-sensitive metadata.
- When checking environment variables, report only variable names and presence, never values.
- Any future write action must require explicit confirmation, create a backup, and be reversible.

## Health Model

Prefer status categories over numeric scores for the first version:

- `ok`
- `needs-review`
- `at-risk`
- `broken`

Basic checks should cover:

- missing or invalid `SKILL.md`
- invalid frontmatter
- weak or missing descriptions
- duplicate names
- broken referenced files
- obvious mojibake
- stale or duplicated cache entries
- Git remote, branch, commit, and dirty state
- MCP server configured but unreachable
- MCP tool/resource/prompt listing failures
- suspicious shell, network, eval, or credential handling patterns

Do not claim to prove whether arbitrary third-party content is malicious.

## Documentation Rules

When updating project docs:

- Keep `skilldesk-project-brief.md` product-facing.
- Keep `AGENTS.md` execution-facing.
- Add future architecture decisions to `DECISIONS.md` only when implementation begins.
- Use `MVP.md` for first-version scope and acceptance criteria.
- Use `DATA_MODEL.md` for scanner, cache, and UI contracts.
- Use `ROADMAP.md` for phase boundaries.
- Use `SECURITY.md` for scanner safety rules.
- Use `SCAFFOLD_PLAN.md` before creating the Tauri project.
- Use `UI_SPEC.md` before implementing frontend views.
- Keep changes surgical and avoid speculative features.
- Preserve Windows examples.
- Preserve the Chinese-first bilingual product decision.
- Run a mojibake check after editing Chinese text.

## Verification

For documentation-only changes, verify:

- Markdown remains readable.
- Chinese text is not garbled.
- Project scope still says read-only MVP.
- The product is clearly not an agent runner.

For implementation changes later, prefer:

```text
npm run typecheck
npm test
npm run lint
```

Only add commands after the project is scaffolded and the scripts exist.
