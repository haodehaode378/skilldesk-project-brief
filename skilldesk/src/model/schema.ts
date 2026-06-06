import { z } from 'zod'

export const platformSchema = z.enum([
  'codex',
  'claude-code',
  'shared',
  'unknown',
])

export const entityKindSchema = z.enum([
  'skill',
  'command',
  'agent',
  'plugin',
  'mcp-server',
  'instruction-file',
])

export const sourceKindSchema = z.enum([
  'local',
  'github',
  'plugin-cache',
  'system',
  'marketplace',
  'unknown',
])

export const healthStatusSchema = z.enum([
  'ok',
  'needs-review',
  'at-risk',
  'broken',
])

export const localeSchema = z.enum(['zh-CN', 'en-US'])

export const issueSeveritySchema = z.enum(['info', 'low', 'medium', 'high'])

export const issueCategorySchema = z.enum([
  'metadata',
  'format',
  'encoding',
  'git',
  'security',
  'size',
  'compatibility',
  'mcp',
  'path',
  'duplication',
])

export const healthIssueSchema = z.object({
  id: z.string().min(1),
  severity: issueSeveritySchema,
  category: issueCategorySchema,
  message: z.string().min(1),
  file: z.string().optional(),
  recommendation: z.string().optional(),
  evidence: z.string().optional(),
})

export const healthSummarySchema = z.object({
  status: healthStatusSchema,
  issues: z.array(healthIssueSchema),
})

export const gitStateSchema = z.object({
  root: z.string().min(1),
  remoteUrl: z.string().optional(),
  branch: z.string().optional(),
  commit: z.string().optional(),
  dirty: z.boolean(),
  ahead: z.number().int().nonnegative().optional(),
  behind: z.number().int().nonnegative().optional(),
  detached: z.boolean().optional(),
  lastCommitMessage: z.string().optional(),
})

const managedEntityBaseSchema = z.object({
  id: z.string().min(1),
  platform: platformSchema,
  name: z.string().min(1),
  title: z.string().optional(),
  description: z.string().optional(),
  path: z.string().min(1),
  source: sourceKindSchema,
  tags: z.array(z.string()),
  discoveredAt: z.string().datetime(),
  lastModified: z.string().datetime().optional(),
  fingerprint: z.string().optional(),
  git: gitStateSchema.optional(),
  health: healthSummarySchema,
})

export const managedSkillSchema = managedEntityBaseSchema.extend({
  kind: z.literal('skill'),
  files: z.object({
    skillMd: z.string().min(1),
    openaiYaml: z.string().optional(),
    claudeMd: z.string().optional(),
    scripts: z.array(z.string()),
    references: z.array(z.string()),
    assets: z.array(z.string()),
  }),
  frontmatter: z.record(z.string(), z.unknown()).optional(),
  declaredVersion: z.string().optional(),
})

export const managedCommandSchema = managedEntityBaseSchema.extend({
  kind: z.literal('command'),
  commandType: z.enum(['slash-command', 'other']),
  file: z.string().min(1),
  namespace: z.string().optional(),
})

export const managedAgentSchema = managedEntityBaseSchema.extend({
  kind: z.literal('agent'),
  file: z.string().min(1),
  declaredTools: z.array(z.string()),
  declaredModel: z.string().optional(),
})

export const managedPluginSchema = managedEntityBaseSchema.extend({
  kind: z.literal('plugin'),
  manifestPath: z.string().optional(),
  version: z.string().optional(),
  publisher: z.string().optional(),
  bundled: z.object({
    skills: z.number().int().nonnegative(),
    commands: z.number().int().nonnegative(),
    agents: z.number().int().nonnegative(),
    mcpServers: z.number().int().nonnegative(),
    hooks: z.number().int().nonnegative(),
  }),
  cache: z.object({
    isCache: z.boolean(),
    isBackup: z.boolean(),
    cacheFamily: z.string().optional(),
  }),
})

export const mcpProbeResultSchema = z.object({
  attempted: z.boolean(),
  reachable: z.boolean().optional(),
  serverName: z.string().optional(),
  serverVersion: z.string().optional(),
  toolsCount: z.number().int().nonnegative().optional(),
  resourcesCount: z.number().int().nonnegative().optional(),
  promptsCount: z.number().int().nonnegative().optional(),
  latencyMs: z.number().nonnegative().optional(),
  error: z.string().optional(),
})

export const managedMcpServerSchema = managedEntityBaseSchema.extend({
  kind: z.literal('mcp-server'),
  configPath: z.string().min(1),
  transport: z.enum(['stdio', 'sse', 'streamable-http', 'http', 'unknown']),
  command: z.string().optional(),
  argsCount: z.number().int().nonnegative().optional(),
  urlHost: z.string().optional(),
  probe: mcpProbeResultSchema.optional(),
})

export const managedInstructionFileSchema = managedEntityBaseSchema.extend({
  kind: z.literal('instruction-file'),
  fileType: z.enum(['AGENTS.md', 'CLAUDE.md', '.mcp.json', 'other']),
  appliesToPath: z.string().min(1),
  lineCount: z.number().int().nonnegative(),
})

export const managedEntitySchema = z.discriminatedUnion('kind', [
  managedSkillSchema,
  managedCommandSchema,
  managedAgentSchema,
  managedPluginSchema,
  managedMcpServerSchema,
  managedInstructionFileSchema,
])

export const scanRootResultSchema = z.object({
  path: z.string().min(1),
  kind: z.enum(['directory', 'file']),
  status: z.enum(['scanned', 'missing', 'skipped', 'error']),
  reason: z.string().optional(),
})

export const scanTotalsSchema = z.object({
  entities: z.number().int().nonnegative(),
  skills: z.number().int().nonnegative(),
  commands: z.number().int().nonnegative(),
  agents: z.number().int().nonnegative(),
  plugins: z.number().int().nonnegative(),
  mcpServers: z.number().int().nonnegative(),
  instructionFiles: z.number().int().nonnegative(),
  byStatus: z.record(healthStatusSchema, z.number().int().nonnegative()),
})

export const scanReportSchema = z.object({
  schemaVersion: z.literal('0.1'),
  generatedAt: z.string().datetime(),
  machine: z.object({
    platform: z.enum(['win32', 'darwin', 'linux', 'unknown']),
    homeDir: z.string().optional(),
  }),
  roots: z.array(scanRootResultSchema),
  entities: z.array(managedEntitySchema),
  totals: scanTotalsSchema,
  issues: z.array(healthIssueSchema),
})

export const appSettingsSchema = z.object({
  locale: localeSchema.default('zh-CN'),
  scanRoots: z.array(z.string()),
  includePluginCaches: z.boolean(),
  mcpProbePolicy: z.enum(['disabled', 'local-only', 'all']),
})

export const skillDeskCacheSchema = z.object({
  cacheVersion: z.literal('0.1'),
  lastReport: scanReportSchema,
})
