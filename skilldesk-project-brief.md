# SkillDesk Project Brief

## Name

**SkillDesk**

Tagline:

```text
Windows-first local skill health dashboard for Codex and Claude Code.
```

Chinese positioning:

```text
SkillDesk: 面向 Codex 和 Claude Code 的本地 Agent 扩展生态体检工作台。
```

Default product language:

```text
Chinese by default, with a Settings switch for Chinese and English.
```

## Why This Name

SkillDesk is short, product-like, and not tied to one agent platform. It suggests a local workbench for viewing, organizing, checking, and maintaining skills. It also avoids the overly generic `skill-manager` name, which is already crowded on GitHub.

Alternative names considered:

- `SkillVault`: better for archive/version storage, less clear as a workbench.
- `AgentSkillDesk`: explicit but longer.
- `SkillShelf`: friendly, but lighter and less operational.
- `SkillClinic`: emphasizes health checks, but sounds narrower.

## Product Definition

SkillDesk is a local desktop-first tool that scans, indexes, reviews, and manages the agent extension ecosystem installed on a developer machine across Codex and Claude Code.

It should focus on local visibility and maintainability first:

- What skills, commands, agents, plugins, MCP servers, and instruction files exist on this machine?
- Which agent platform can use each extension?
- Where did each extension come from?
- Is the extension healthy, stale, duplicated, dirty, or unsafe?
- Is it backed by GitHub, a plugin cache, a system bundle, or local manual edits?
- What changed between versions?

SkillDesk is not primarily a public marketplace, a generic package manager, or an agent runner. Its first job is to make the local agent extension ecosystem understandable and maintainable.

Boundary against broader agent workspaces:

```text
Elves runs agents.
SkillDesk audits the local ecosystem agents depend on.
```

## Target Users

- Developers using both Codex and Claude Code.
- Users with many local skills spread across multiple folders.
- Users who install skills from GitHub and need version/update visibility.
- Skill authors who need health checks before publishing or reusing skills.
- Power users who want a Windows-friendly local dashboard instead of terminal-only management.

## Core Scope

SkillDesk should manage:

- Codex skills.
- Claude Code skills.
- Claude Code slash commands.
- Claude Code agents.
- Codex and Claude Code plugins.
- MCP server configurations and lightweight health status.
- Project instruction files such as `AGENTS.md`, `CLAUDE.md`, and `.mcp.json`.
- Local manually created skills.
- GitHub-backed skills.
- Plugin-cache or system-installed skills.
- Version, source, health, and update state.

Initial supported locations should include configurable scan roots, with defaults such as:

```text
C:\Users\<user>\.codex\skills
C:\Users\<user>\.agents\skills
C:\Users\<user>\.claude\skills
C:\Users\<user>\.claude\commands
C:\Users\<user>\.claude\agents
C:\Users\<user>\.codex\config.toml
C:\Users\<user>\.claude\mcp-configs
```

Plugin cache directories should not be deeply scanned by default because they can be nested, duplicated, backed up, and noisy. The MVP may include optional summary-level plugin cache scanning for plugin manifests and bundled skill counts.

## Non-Goals

For the first version, SkillDesk should not:

- Auto-update skills without review.
- Execute skill scripts.
- Treat third-party skill instructions as trusted commands.
- Become a full public marketplace.
- Manage every AI agent ecosystem at once.
- Run or orchestrate agents.
- Create git worktrees or embedded agent terminals.
- Manage memory, session replay, or task execution.
- Overwrite Codex or Claude Code files without explicit confirmation.
- Require cloud accounts.
- Depend on image generation for core functionality.

## Main Differentiation

Existing tools already cover parts of this space:

- Claude-focused skill UIs.
- Cross-agent package managers.
- Registry/install tools.
- MCP and slash-command managers.

SkillDesk should differentiate by being:

- **Windows-first**: strong support for Windows paths, PowerShell, and local desktop workflows.
- **Chinese-first bilingual UI**: Chinese is the default interface language, with English available from Settings.
- **Codex + Claude Code first**: not too broad in the MVP.
- **Health-dashboard first**: scanning, validation, quality checks, version state, and review.
- **Git-aware**: track GitHub-backed skills, dirty state, branch, commit, ahead/behind, and update availability.
- **Visual and usable**: use generated UI assets for skill cards, categories, and marketplace previews without making image generation the product itself.

## Data Model

Each detected skill should normalize into a shared model.

```ts
type ManagedSkill = {
  id: string;
  name: string;
  platform: "codex" | "claude-code" | "shared" | "unknown";
  path: string;
  source: "local" | "github" | "plugin-cache" | "system" | "unknown";
  title?: string;
  description?: string;
  tags: string[];
  files: {
    skillMd?: string;
    openaiYaml?: string;
    claudeMd?: string;
    scripts: string[];
    references: string[];
    assets: string[];
  };
  git?: {
    remoteUrl?: string;
    branch?: string;
    commit?: string;
    dirty: boolean;
    ahead?: number;
    behind?: number;
    lastCommitMessage?: string;
  };
  version?: {
    declared?: string;
    fingerprint: string;
    lastModified: string;
  };
  health: {
    score: number;
    status: "healthy" | "needs-review" | "at-risk" | "broken";
    issues: SkillIssue[];
  };
};
```

Health issues should be structured:

```ts
type SkillIssue = {
  severity: "info" | "low" | "medium" | "high";
  category:
    | "metadata"
    | "format"
    | "encoding"
    | "git"
    | "security"
    | "size"
    | "compatibility";
  message: string;
  file?: string;
  recommendation?: string;
};
```

## Health Checks

Codex skill checks:

- `SKILL.md` exists.
- YAML frontmatter is valid.
- `name` exists and uses lowercase hyphen-case.
- `description` exists and clearly states when to use the skill.
- Description is not excessively long.
- `agents/openai.yaml` exists when expected.
- `agents/openai.yaml` appears consistent with `SKILL.md`.
- No obvious mojibake such as garbled CJK text or replacement characters.
- No obvious secrets or machine-specific absolute paths in public-facing metadata.
- Large references are mentioned in `SKILL.md` with clear loading guidance.
- Scripts are present only when they serve deterministic or repeated workflows.

Claude Code checks:

- Skill or command files exist in expected locations.
- Metadata or description is discoverable.
- Command names do not conflict.
- Files do not contain obvious mojibake.
- Files do not include suspicious shell/network/eval patterns.

Git-backed skill checks:

- Remote URL is detected.
- Current branch and commit are detected.
- Dirty worktree is shown.
- Ahead/behind state is detected when upstream exists.
- Detached HEAD is flagged.
- Local modifications are never overwritten automatically.

## Version Strategy

SkillDesk should support three version layers.

### 1. Git Version

Best for GitHub-backed skills.

Track:

- remote URL
- branch
- commit
- tag if available
- dirty state
- ahead/behind
- latest fetched state

### 2. Declared Version

Optional metadata from:

- `SKILL.md` frontmatter extension
- `skill.json`
- `package.json`
- `.codex-skill.json`

Declared versions are helpful but not required by Codex skill standards.

### 3. Content Fingerprint

For local-only skills, compute a stable fingerprint from:

- `SKILL.md`
- `agents/openai.yaml`
- key scripts
- key reference file names and sizes

This does not replace semantic versioning, but it shows whether the skill changed.

## UI Structure

### Overview

Show high-level local health:

```text
Total Skills
Codex Skills
Claude Code Skills
GitHub-backed Skills
Local-only Skills
Needs Review
Dirty Git Skills
Updates Available
Possible Mojibake
```

### Skills

Searchable table and card view:

- name
- platform
- source
- health
- version
- Git status
- last modified
- tags

### Skill Detail

Show:

- metadata
- file tree
- `SKILL.md` preview
- `agents/openai.yaml` preview
- Claude command/agent files when relevant
- health issues
- Git state
- version history or fingerprint history
- recommended next actions

### Sources

Manage scan roots:

- default Codex roots
- default Claude Code roots
- custom local roots
- GitHub clone roots

### Updates

Focused view for Git-backed skills:

- clean
- dirty
- behind remote
- ahead remote
- detached
- remote unavailable

MVP should show update state only. Pull/update actions should come later.

## Use Of Generated UI Assets

Codex image generation should support the UI, not replace the manager.

Useful generated assets:

- Skill card covers.
- Category icons.
- Platform identity illustrations.
- Health-state visuals.
- Marketplace preview images.
- Empty-state illustrations.

Rules:

- Images should never contain critical factual text.
- The UI should render names, versions, and warnings as real text.
- Image prompts must not include secrets, absolute paths, or private local details.
- Generated images should be cached and referenced through an asset manifest.
- Missing images must not block scanning or health checks.

Example card prompt:

```text
Create a clean product-style card illustration for a local AI agent skill named "nature-paper2ppt".
The skill helps transform scientific papers into presentation materials.
Style: polished developer productivity UI, academic but modern, calm colors, no readable text inside the image, no logos, no mascots.
Purpose: small card cover in a local skill management dashboard.
```

## MVP

The first version should be read-only.

MVP requirements:

- Scan configurable local directories.
- Default the interface to Chinese.
- Provide a Settings language switch between Chinese and English.
- Detect Codex skills from `SKILL.md`.
- Detect Claude Code skill/command/agent folders where present.
- Detect plugin manifests and plugin-bundled skills at summary level.
- Detect MCP server configuration names, sources, transport type, and lightweight health status.
- Detect project instruction files such as `AGENTS.md`, `CLAUDE.md`, and `.mcp.json`.
- Parse `SKILL.md` frontmatter.
- Detect `agents/openai.yaml`.
- Detect Git source, branch, commit, dirty state.
- Compute content fingerprint.
- Run basic health checks.
- Detect likely mojibake.
- Show dashboard, list, and detail page.
- Generate image prompts for skill cards.
- Do not run skill scripts.
- Do not run hooks, plugin commands, or agent tasks.
- Do not update or overwrite skills.

## V2

Second version can add controlled write actions:

- Fetch remote Git state.
- Show diff for Git-backed skills.
- Pull/update after confirmation.
- Create local snapshot before update.
- Generate or refresh `agents/openai.yaml`.
- Generate skill card images.
- Export health report.
- Detect duplicate skills.
- Add project-level install/deploy matrix.

## Recommended Tech Stack

For a maintainable Windows-first local tool:

- TypeScript.
- Node.js 20+.
- React.
- Vite.
- Tauri.
- JSON cache first, SQLite after the scan/index model stabilizes.
- `commander` for optional CLI.
- `zod` for schemas.
- `simple-git` or direct `git` commands for Git metadata.
- `vitest` for tests.

Delivery options:

- Start as a Tauri desktop app with a reusable scanner core.
- Keep the scanner core CLI-friendly so a local web UI and reports can reuse it later.

## Initial Commands

Potential CLI shape:

```bash
skilldesk scan
skilldesk start
skilldesk report --out ./skilldesk-report
skilldesk refresh-git
```

Potential local web URL:

```text
http://localhost:4878
```

## Success Criteria

The MVP is useful when:

- It finds all major Codex and Claude Code skills on the machine.
- It distinguishes local, system, plugin-cache, and GitHub-backed skills.
- It shows which skills need review and why.
- It surfaces dirty or stale Git-backed skills.
- It catches obvious format and encoding problems.
- It gives enough context to maintain skills without digging through folders manually.
- Generated UI assets improve recognition without becoming required for correctness.
