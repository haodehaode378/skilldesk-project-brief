import { appCopy } from '../app/i18n'
import type { EntityKind, HealthStatus, IssueSeverity } from '../model'
import {
  entityKindFilters,
  issueSeverityFilters,
  statusFilters,
  type EntityKindFilter,
  type IssueSeverityFilter,
  type StatusFilter,
} from '../features/viewState'

function formatStatus(status: HealthStatus, copy: typeof appCopy['zh-CN']) {
  const labels: Record<HealthStatus, string> = {
    ok: copy.labels.statusOk,
    'needs-review': copy.labels.statusNeedsReview,
    'at-risk': copy.labels.statusAtRisk,
    broken: copy.labels.statusBroken,
  }

  return labels[status]
}

function formatSeverity(severity: IssueSeverity, copy: typeof appCopy['zh-CN']) {
  const labels: Record<IssueSeverity, string> = {
    info: copy.labels.severityInfo,
    low: copy.labels.severityLow,
    medium: copy.labels.severityMedium,
    high: copy.labels.severityHigh,
  }

  return labels[severity]
}

function formatKind(kind: EntityKind, copy: typeof appCopy['zh-CN']) {
  const labels: Record<EntityKind, string> = {
    skill: copy.labels.kindSkill,
    command: copy.labels.kindCommand,
    agent: copy.labels.kindAgent,
    plugin: copy.labels.kindPlugin,
    'mcp-server': copy.labels.kindMcpServer,
    'instruction-file': copy.labels.kindInstructionFile,
  }

  return labels[kind]
}

export function SectionHeading({
  title,
  body,
}: {
  title: string
  body?: string
}) {
  return (
    <header className="section-heading">
      <h3>{title}</h3>
      {body && <p>{body}</p>}
    </header>
  )
}

export function EmptyState({ message }: { message: string }) {
  return <p className="empty-state">{message}</p>
}

export function StatusBadge({
  copy,
  status,
}: {
  copy: typeof appCopy['zh-CN']
  status: HealthStatus
}) {
  return <span className={`status-badge status-${status}`}>{formatStatus(status, copy)}</span>
}

export function StatusFilterBar({
  copy,
  activeStatus,
  onChange,
}: {
  copy: typeof appCopy['zh-CN']
  activeStatus: StatusFilter
  onChange: (status: StatusFilter) => void
}) {
  return (
    <div className="filter-bar" aria-label={copy.labels.status}>
      {statusFilters.map((status) => (
        <button
          key={status}
          type="button"
          className="filter-button"
          aria-pressed={activeStatus === status}
          onClick={() => onChange(status)}
        >
          {status === 'all' ? copy.labels.allStatuses : formatStatus(status, copy)}
        </button>
      ))}
    </div>
  )
}

export function EntityKindFilterBar({
  copy,
  activeKind,
  onChange,
}: {
  copy: typeof appCopy['zh-CN']
  activeKind: EntityKindFilter
  onChange: (kind: EntityKindFilter) => void
}) {
  return (
    <div className="filter-bar" aria-label={copy.labels.kind}>
      {entityKindFilters.map((kind) => (
        <button
          key={kind}
          type="button"
          className="filter-button"
          aria-pressed={activeKind === kind}
          onClick={() => onChange(kind)}
        >
          {kind === 'all' ? copy.labels.allKinds : formatKind(kind, copy)}
        </button>
      ))}
    </div>
  )
}

export function IssueSeverityFilterBar({
  copy,
  activeSeverity,
  onChange,
}: {
  copy: typeof appCopy['zh-CN']
  activeSeverity: IssueSeverityFilter
  onChange: (severity: IssueSeverityFilter) => void
}) {
  return (
    <div className="filter-bar" aria-label={copy.labels.severity}>
      {issueSeverityFilters.map((severity) => (
        <button
          key={severity}
          type="button"
          className="filter-button"
          aria-pressed={activeSeverity === severity}
          onClick={() => onChange(severity)}
        >
          {severity === 'all'
            ? copy.labels.allSeverities
            : formatSeverity(severity, copy)}
        </button>
      ))}
    </div>
  )
}
