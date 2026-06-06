# SkillDesk Scaffold Plan

This plan should be followed when the user explicitly asks to begin implementation.

## Recommended Tooling

Preferred stack:

```text
Tauri + React + Vite + TypeScript
```

Confirmed package manager:

```text
pnpm
```

Use `pnpm`.

## Repository Decision Before Scaffolding

Confirmed:

- Planning documents may be committed to `https://github.com/haodehaode378/skilldesk-project-brief.git`.
- Implementation code should live in a new subfolder under `E:\ai-skilldesk-project-brief`.

Current safe default:

- Keep planning files at the repository root.
- Create the app under a new implementation subfolder.
- Do not scaffold until the user explicitly asks to begin implementation.

## Suggested Directory Shape

If implementation starts in this folder, use:

```text
.
├─ skilldesk/
│  ├─ src/
│  │  ├─ app/
│  │  ├─ components/
│  │  ├─ features/
│  │  ├─ scanner/
│  │  ├─ model/
│  │  ├─ fixtures/
│  │  └─ test/
│  ├─ src-tauri/
│  ├─ public/
│  ├─ package.json
│  ├─ vite.config.ts
│  └─ tsconfig.json
├─ AGENTS.md
├─ MVP.md
├─ DATA_MODEL.md
├─ SECURITY.md
├─ ROADMAP.md
└─ DECISIONS.md
```

Preferred source boundaries inside the implementation subfolder:

- `src/model`: shared TypeScript types and zod schemas.
- `src/scanner`: read-only scanner core.
- `src/features`: UI feature modules.
- `src/components`: shared UI components.
- `src/fixtures`: safe fake scan reports for UI and tests.
- `src-tauri`: Tauri shell and IPC bridge.

## Scanner Core Boundary

The scanner should be usable without the Tauri UI.

Target API:

```ts
type ScanOptions = {
  roots: string[];
  includePluginCaches?: boolean;
  probeMcp?: boolean;
};

async function scan(options: ScanOptions): Promise<ScanReport>;
```

Target CLI:

```bash
skilldesk scan --json
skilldesk scan --root "%USERPROFILE%\.codex\skills" --json
```

The Tauri app should call this same scanner boundary rather than duplicate scan logic.

## Initial Implementation Order

1. Create Tauri + React + TypeScript app.
2. Add shared model types and zod schemas.
3. Add fixture `ScanReport`.
4. Build UI from fixture data.
5. Implement scanner root discovery and file walking.
6. Implement skill parsing.
7. Implement command and agent discovery.
8. Implement MCP config discovery without probes.
9. Add optional local MCP probe boundary.
10. Wire scanner result into UI.

## First Tests

Add tests before broad scanning:

- Windows path normalization.
- Default root expansion.
- Exclusion matching.
- Frontmatter parsing.
- Mojibake detection.
- Health issue aggregation.
- Scan report totals.

## Commands To Expect After Scaffold

Exact scripts may vary, but the project should eventually support:

```bash
pnpm dev
pnpm tauri dev
pnpm test
pnpm typecheck
pnpm lint
```

If using `npm`, use equivalent `npm run ...` commands.

## Do Not Do During Scaffold

- Do not add write/update actions.
- Do not add agent execution.
- Do not add embedded terminal features.
- Do not deeply scan plugin caches by default.
- Do not call arbitrary MCP tools.
- Do not commit secrets or local auth files.
