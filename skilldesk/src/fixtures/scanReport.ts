import { calculateScanTotals } from '../model/totals'
import type { ManagedEntity, ScanReport } from '../model/types'

const discoveredAt = '2026-06-06T00:00:00.000Z'

export const fixtureEntities: ManagedEntity[] = [
  {
    id: 'skill:codex:C:/Users/example/.codex/skills/openai-docs/SKILL.md',
    kind: 'skill',
    platform: 'codex',
    name: 'openai-docs',
    title: 'OpenAI Docs',
    description: 'Use when official OpenAI API documentation is needed.',
    path: 'C:\\Users\\example\\.codex\\skills\\openai-docs',
    source: 'system',
    tags: ['docs', 'openai'],
    discoveredAt,
    files: {
      skillMd: 'C:\\Users\\example\\.codex\\skills\\openai-docs\\SKILL.md',
      scripts: [],
      references: [],
      assets: [],
    },
    health: {
      status: 'ok',
      issues: [],
    },
  },
  {
    id: 'command:claude-code:C:/Users/example/.claude/commands/ship.md',
    kind: 'command',
    platform: 'claude-code',
    name: 'ship',
    title: 'Ship',
    path: 'C:\\Users\\example\\.claude\\commands\\ship.md',
    source: 'local',
    tags: ['command'],
    discoveredAt,
    commandType: 'slash-command',
    file: 'C:\\Users\\example\\.claude\\commands\\ship.md',
    health: {
      status: 'needs-review',
      issues: [
        {
          id: 'issue:command:ship:metadata-description',
          severity: 'low',
          category: 'metadata',
          message: 'Command description is missing.',
          file: 'C:\\Users\\example\\.claude\\commands\\ship.md',
          recommendation: 'Add a short description of when to use this command.',
        },
      ],
    },
  },
  {
    id: 'mcp-server:codex:openpencil',
    kind: 'mcp-server',
    platform: 'codex',
    name: 'openpencil',
    title: 'OpenPencil',
    path: 'C:\\Users\\example\\.codex\\config.toml',
    source: 'local',
    tags: ['mcp', 'design'],
    discoveredAt,
    configPath: 'C:\\Users\\example\\.codex\\config.toml',
    transport: 'stdio',
    command: 'node.exe',
    argsCount: 1,
    probe: {
      attempted: true,
      reachable: true,
      serverName: '@zseven-w/pen-mcp',
      serverVersion: '0.6.0',
      toolsCount: 42,
      resourcesCount: 0,
      promptsCount: 0,
      latencyMs: 84,
    },
    health: {
      status: 'ok',
      issues: [],
    },
  },
  {
    id: 'plugin:codex:openai-bundled/browser',
    kind: 'plugin',
    platform: 'codex',
    name: 'browser',
    title: 'Browser',
    path: 'C:\\Users\\example\\.codex\\plugins\\cache\\openai-bundled\\browser',
    source: 'plugin-cache',
    tags: ['plugin', 'cache'],
    discoveredAt,
    bundled: {
      skills: 1,
      commands: 0,
      agents: 0,
      mcpServers: 1,
      hooks: 0,
    },
    cache: {
      isCache: true,
      isBackup: false,
      cacheFamily: 'openai-bundled',
    },
    health: {
      status: 'needs-review',
      issues: [
        {
          id: 'issue:plugin:browser:cache-summary',
          severity: 'info',
          category: 'metadata',
          message: 'Plugin cache entry is summarized but not deeply scanned.',
          recommendation: 'Enable advanced plugin cache scanning if needed.',
        },
      ],
    },
  },
]

const issues = fixtureEntities.flatMap((entity) => entity.health.issues)

export const fixtureScanReport: ScanReport = {
  schemaVersion: '0.1',
  generatedAt: discoveredAt,
  machine: {
    platform: 'win32',
    homeDir: 'C:\\Users\\example',
  },
  roots: [
    {
      path: 'C:\\Users\\example\\.codex\\skills',
      kind: 'directory',
      status: 'scanned',
    },
    {
      path: 'C:\\Users\\example\\.claude\\mcp-configs',
      kind: 'directory',
      status: 'missing',
      reason: 'Directory does not exist.',
    },
  ],
  entities: fixtureEntities,
  totals: calculateScanTotals(fixtureEntities),
  issues,
}
