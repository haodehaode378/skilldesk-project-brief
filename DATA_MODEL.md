# SkillDesk Data Model

This file defines the initial normalized data model for the scanner, cache, and UI. It is intentionally conservative and may evolve once implementation begins.

## Core Types

```ts
type Platform = "codex" | "claude-code" | "shared" | "unknown";

type EntityKind =
  | "skill"
  | "command"
  | "agent"
  | "plugin"
  | "mcp-server"
  | "instruction-file";

type SourceKind =
  | "local"
  | "github"
  | "plugin-cache"
  | "system"
  | "marketplace"
  | "unknown";

type HealthStatus = "ok" | "needs-review" | "at-risk" | "broken";

type IssueSeverity = "info" | "low" | "medium" | "high";

type IssueCategory =
  | "metadata"
  | "format"
  | "encoding"
  | "git"
  | "security"
  | "size"
  | "compatibility"
  | "mcp"
  | "path"
  | "duplication";
```

## Base Entity

Every discovered object should normalize to a base entity shape.

```ts
type ManagedEntityBase = {
  id: string;
  kind: EntityKind;
  platform: Platform;
  name: string;
  title?: string;
  description?: string;
  path: string;
  source: SourceKind;
  tags: string[];
  discoveredAt: string;
  lastModified?: string;
  fingerprint?: string;
  git?: GitState;
  health: HealthSummary;
};
```

ID guidance:

- Use stable IDs derived from `kind`, normalized absolute path, and platform.
- Do not use array index IDs.
- Keep raw paths out of public image prompts.

## Health

```ts
type HealthSummary = {
  status: HealthStatus;
  issues: HealthIssue[];
};

type HealthIssue = {
  id: string;
  severity: IssueSeverity;
  category: IssueCategory;
  message: string;
  file?: string;
  recommendation?: string;
  evidence?: string;
};
```

Guidance:

- Avoid numeric scores in the first version.
- Keep issue messages actionable.
- Do not include secret values in `evidence`.

## Git State

```ts
type GitState = {
  root: string;
  remoteUrl?: string;
  branch?: string;
  commit?: string;
  dirty: boolean;
  ahead?: number;
  behind?: number;
  detached?: boolean;
  lastCommitMessage?: string;
};
```

## Managed Skill

```ts
type ManagedSkill = ManagedEntityBase & {
  kind: "skill";
  files: {
    skillMd: string;
    openaiYaml?: string;
    claudeMd?: string;
    scripts: string[];
    references: string[];
    assets: string[];
  };
  frontmatter?: Record<string, unknown>;
  declaredVersion?: string;
};
```

## Managed Command

```ts
type ManagedCommand = ManagedEntityBase & {
  kind: "command";
  commandType: "slash-command" | "other";
  file: string;
  namespace?: string;
};
```

## Managed Agent

```ts
type ManagedAgent = ManagedEntityBase & {
  kind: "agent";
  file: string;
  declaredTools: string[];
  declaredModel?: string;
};
```

## Managed Plugin

```ts
type ManagedPlugin = ManagedEntityBase & {
  kind: "plugin";
  manifestPath?: string;
  version?: string;
  publisher?: string;
  bundled: {
    skills: number;
    commands: number;
    agents: number;
    mcpServers: number;
    hooks: number;
  };
  cache: {
    isCache: boolean;
    isBackup: boolean;
    cacheFamily?: string;
  };
};
```

## Managed MCP Server

```ts
type ManagedMcpServer = ManagedEntityBase & {
  kind: "mcp-server";
  configPath: string;
  transport: "stdio" | "sse" | "streamable-http" | "http" | "unknown";
  command?: string;
  argsCount?: number;
  urlHost?: string;
  probe?: McpProbeResult;
};

type McpProbeResult = {
  attempted: boolean;
  reachable?: boolean;
  serverName?: string;
  serverVersion?: string;
  toolsCount?: number;
  resourcesCount?: number;
  promptsCount?: number;
  latencyMs?: number;
  error?: string;
};
```

MCP safety:

- Store command name and argument count, not full secret-bearing arguments.
- Store URL host, not full URL when it may contain credentials.
- Do not call arbitrary MCP tools in the MVP.

## Managed Instruction File

```ts
type ManagedInstructionFile = ManagedEntityBase & {
  kind: "instruction-file";
  fileType: "AGENTS.md" | "CLAUDE.md" | ".mcp.json" | "other";
  appliesToPath: string;
  lineCount: number;
};
```

## Scan Report

```ts
type ScanReport = {
  schemaVersion: "0.1";
  generatedAt: string;
  machine: {
    platform: "win32" | "darwin" | "linux" | "unknown";
    homeDir?: string;
  };
  roots: ScanRootResult[];
  entities: ManagedEntity[];
  totals: ScanTotals;
  issues: HealthIssue[];
};

type ManagedEntity =
  | ManagedSkill
  | ManagedCommand
  | ManagedAgent
  | ManagedPlugin
  | ManagedMcpServer
  | ManagedInstructionFile;

type ScanRootResult = {
  path: string;
  kind: "directory" | "file";
  status: "scanned" | "missing" | "skipped" | "error";
  reason?: string;
};

type ScanTotals = {
  entities: number;
  skills: number;
  commands: number;
  agents: number;
  plugins: number;
  mcpServers: number;
  instructionFiles: number;
  byStatus: Record<HealthStatus, number>;
};
```

## Cache Shape

The first cache should be a JSON file containing:

```ts
type SkillDeskCache = {
  cacheVersion: "0.1";
  lastReport: ScanReport;
};
```

Future SQLite storage can preserve the same logical model.
