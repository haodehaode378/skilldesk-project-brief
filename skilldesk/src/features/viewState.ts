import { appCopy } from '../app/i18n'
import type { ReportSource } from '../app/exportReport'
import type { EntityKind, HealthStatus, IssueSeverity } from '../model'

export type ViewKey =
  | 'overview'
  | 'extensions'
  | 'mcp'
  | 'plugins'
  | 'sources'
  | 'issues'
  | 'settings'

export const viewOrder: ViewKey[] = [
  'overview',
  'extensions',
  'mcp',
  'plugins',
  'sources',
  'issues',
  'settings',
]

export type StatusFilter = HealthStatus | 'all'
export type EntityKindFilter = EntityKind | 'all'
export type IssueSeverityFilter = IssueSeverity | 'all'
export type ScanState = ReportSource

export const statusFilters: StatusFilter[] = [
  'all',
  'ok',
  'needs-review',
  'at-risk',
  'broken',
]

export const entityKindFilters: EntityKindFilter[] = [
  'all',
  'skill',
  'command',
  'agent',
  'plugin',
  'mcp-server',
  'instruction-file',
]

export const issueSeverityFilters: IssueSeverityFilter[] = [
  'all',
  'info',
  'low',
  'medium',
  'high',
]

export const navKeyByView: Record<ViewKey, keyof typeof appCopy['zh-CN']['nav']> = {
  overview: 'overview',
  extensions: 'extensions',
  mcp: 'mcpServers',
  plugins: 'plugins',
  sources: 'sources',
  issues: 'issues',
  settings: 'settings',
}
