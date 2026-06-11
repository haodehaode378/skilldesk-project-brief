# SkillDesk Decisions

This file records project decisions that should guide implementation once the app is scaffolded.

## D001: Build A Desktop-First App

Decision: use `Tauri + React + Vite + TypeScript`.

Rationale:

- SkillDesk is a local Windows-first tool.
- A desktop shell can access local paths and present a stable dashboard without requiring a hosted service.
- React and Vite keep the UI fast to build and easy to prototype.
- TypeScript keeps scanner output, cache data, and UI contracts explicit.

Consequences:

- The scanner core must stay separate from UI components.
- Tauri commands should call into a small backend boundary rather than contain scanning logic directly.
- The app must still support a CLI-friendly scanner path so reports and future web views can reuse the same model.

## D002: SkillDesk Is Not An Agent Runner

Decision: SkillDesk audits local agent extension infrastructure; it does not run agents.

Rationale:

- Existing tools such as Elves focus on orchestration, worktrees, terminals, and execution.
- SkillDesk's narrower value is inventory, health, drift, duplication, and risk visibility.
- Avoiding execution keeps the MVP safer and easier to verify.

Non-goals:

- No worktree orchestration.
- No embedded terminal.
- No task execution.
- No memory or session replay.
- No agent runtime control.

## D003: MVP Is Read-Only

Decision: the first version must not mutate local skill, plugin, MCP, or agent files.

Rationale:

- Local agent extensions may contain untrusted scripts, hooks, and command definitions.
- A health dashboard is useful before any write actions exist.
- Read-only behavior makes it safer to scan broad local roots.

Allowed:

- File discovery.
- Text parsing.
- Metadata extraction.
- Hashing.
- Git metadata reads.
- MCP protocol list calls when explicitly treated as lightweight read-only probes.

Disallowed:

- Running skill scripts.
- Running hooks.
- Running plugin commands.
- Calling arbitrary MCP tools.
- Installing, updating, deleting, or rewriting local extensions.

## D004: Scanner Core Is CLI-Friendly

Decision: implement the scanner as a reusable Rust core with a CLI-compatible JSON boundary.

Rationale:

- The Tauri desktop app is the primary product, but scan logic should not be trapped in UI code.
- Rust keeps local filesystem scanning close to the Tauri backend and avoids running local extension content through Node.
- A CLI-compatible scanner enables JSON reports, tests, and future automation.
- The same scan report should feed desktop UI, local web UI, and report export.

Expected shape:

```text
scan roots -> normalized entities -> health issues -> scan report -> cache/UI
```

## D005: JSON Cache First, SQLite Later

Decision: start with a JSON cache; add SQLite only after the scan/index model stabilizes.

Rationale:

- The data model is still evolving.
- JSON snapshots are easy to inspect and compare during early development.
- SQLite becomes useful after historical trends, filtering, and larger datasets are proven necessary.

## D006: MCP Is Scanned, Not Used As A General Tool Surface

Decision: MCP servers belong in the MVP inventory, but the MVP only performs lightweight protocol discovery.

Allowed MCP probes:

- `initialize`
- `tools/list`
- `resources/list`
- `prompts/list`

Disallowed:

- Calling arbitrary tools.
- Reading secret-bearing resources.
- Sending user files to MCP servers.
- Treating tool results as trusted commands.

## D007: Plugin Cache Is Summary-Level By Default

Decision: plugin caches are optional advanced scan sources and should not be deeply scanned by default.

Rationale:

- Plugin caches can contain duplicate versions, backups, marketplace expansions, runtime bundles, and `node_modules`.
- Deep scanning cache trees can distort local inventory counts.
- Summary-level manifest and bundled skill counts are enough for the MVP.

## D008: Health Uses Status Categories First

Decision: use status categories before numeric scoring.

Statuses:

- `ok`
- `needs-review`
- `at-risk`
- `broken`

Rationale:

- Numeric health scores imply precision the MVP will not have.
- Status plus concrete issues is more actionable.
- Scores can be added later once issue weights are validated.

## D009: OpenPencil Is A Design Aid

Decision: use OpenPencil for UI prototyping and design exploration, not as a runtime dependency.

Rationale:

- OpenPencil can help shape the dashboard and detail views before implementation.
- The app must remain functional without OpenPencil.

## Open Decisions

- Package manager: `npm`, `pnpm`, or another tool.
- Tauri backend implementation split: Rust-only file access vs. Node scanner subprocess.
- Whether MCP lightweight probes run by default or require an explicit toggle.
- Exact UI density and visual style after OpenPencil prototyping.
