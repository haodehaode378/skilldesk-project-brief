export type Platform = 'codex' | 'claude-code' | 'shared' | 'unknown'

export type EntityKind =
  | 'skill'
  | 'command'
  | 'agent'
  | 'plugin'
  | 'mcp-server'
  | 'instruction-file'

export type SourceKind =
  | 'local'
  | 'github'
  | 'plugin-cache'
  | 'system'
  | 'marketplace'
  | 'unknown'

export type HealthStatus = 'ok' | 'needs-review' | 'at-risk' | 'broken'

export type Locale = 'zh-CN' | 'en-US'

export type IssueSeverity = 'info' | 'low' | 'medium' | 'high'

export type IssueCategory =
  | 'metadata'
  | 'format'
  | 'encoding'
  | 'git'
  | 'security'
  | 'size'
  | 'compatibility'
  | 'mcp'
  | 'path'
  | 'duplication'

export type HealthIssue = {
  id: string
  severity: IssueSeverity
  category: IssueCategory
  message: string
  file?: string
  recommendation?: string
  evidence?: string
}

export type HealthSummary = {
  status: HealthStatus
  issues: HealthIssue[]
}

export type GitState = {
  root: string
  remoteUrl?: string
  branch?: string
  commit?: string
  dirty: boolean
  ahead?: number
  behind?: number
  detached?: boolean
  lastCommitMessage?: string
}

export type ManagedEntityBase = {
  id: string
  kind: EntityKind
  platform: Platform
  name: string
  title?: string
  description?: string
  path: string
  source: SourceKind
  tags: string[]
  discoveredAt: string
  lastModified?: string
  fingerprint?: string
  git?: GitState
  health: HealthSummary
}

export type ManagedSkill = ManagedEntityBase & {
  kind: 'skill'
  files: {
    skillMd: string
    openaiYaml?: string
    claudeMd?: string
    scripts: string[]
    references: string[]
    assets: string[]
  }
  frontmatter?: Record<string, unknown>
  declaredVersion?: string
}

export type ManagedCommand = ManagedEntityBase & {
  kind: 'command'
  commandType: 'slash-command' | 'other'
  file: string
  namespace?: string
}

export type ManagedAgent = ManagedEntityBase & {
  kind: 'agent'
  file: string
  declaredTools: string[]
  declaredModel?: string
}

export type ManagedPlugin = ManagedEntityBase & {
  kind: 'plugin'
  manifestPath?: string
  version?: string
  publisher?: string
  bundled: {
    skills: number
    commands: number
    agents: number
    mcpServers: number
    hooks: number
  }
  cache: {
    isCache: boolean
    isBackup: boolean
    cacheFamily?: string
  }
}

export type McpProbeResult = {
  attempted: boolean
  reachable?: boolean
  serverName?: string
  serverVersion?: string
  toolsCount?: number
  resourcesCount?: number
  promptsCount?: number
  latencyMs?: number
  error?: string
}

export type ManagedMcpServer = ManagedEntityBase & {
  kind: 'mcp-server'
  configPath: string
  transport: 'stdio' | 'sse' | 'streamable-http' | 'http' | 'unknown'
  command?: string
  argsCount?: number
  urlHost?: string
  probe?: McpProbeResult
}

export type ManagedInstructionFile = ManagedEntityBase & {
  kind: 'instruction-file'
  fileType: 'AGENTS.md' | 'CLAUDE.md' | '.mcp.json' | 'other'
  appliesToPath: string
  lineCount: number
}

export type ManagedEntity =
  | ManagedSkill
  | ManagedCommand
  | ManagedAgent
  | ManagedPlugin
  | ManagedMcpServer
  | ManagedInstructionFile

export type ScanRootResult = {
  path: string
  kind: 'directory' | 'file'
  status: 'scanned' | 'missing' | 'skipped' | 'error'
  reason?: string
}

export type ScanTotals = {
  entities: number
  skills: number
  commands: number
  agents: number
  plugins: number
  mcpServers: number
  instructionFiles: number
  byStatus: Record<HealthStatus, number>
}

export type ScanReport = {
  schemaVersion: '0.1'
  generatedAt: string
  machine: {
    platform: 'win32' | 'darwin' | 'linux' | 'unknown'
    homeDir?: string
  }
  roots: ScanRootResult[]
  entities: ManagedEntity[]
  totals: ScanTotals
  issues: HealthIssue[]
}

export type AppSettings = {
  locale: Locale
  scanRoots: string[]
  includePluginCaches: boolean
  mcpProbePolicy: 'disabled' | 'local-only' | 'all'
}

export type SkillDeskCache = {
  cacheVersion: '0.1'
  lastReport: ScanReport
}
