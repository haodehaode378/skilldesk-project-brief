import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

import {
  loadCachedReport,
  loadLocale,
  saveCachedReport,
  saveLocale,
} from './app/cache'
import { appCopy, nextLocale } from './app/i18n'
import './App.css'
import { fixtureScanReport } from './fixtures'
import { defaultAppSettings, scanReportSchema } from './model'
import type {
  AppSettings,
  EntityKind,
  HealthStatus,
  HealthIssue,
  IssueSeverity,
  Locale,
  ManagedEntity,
  ScanReport,
} from './model'

type ViewKey =
  | 'overview'
  | 'extensions'
  | 'mcp'
  | 'plugins'
  | 'sources'
  | 'issues'
  | 'settings'

const viewOrder: ViewKey[] = [
  'overview',
  'extensions',
  'mcp',
  'plugins',
  'sources',
  'issues',
  'settings',
]

type StatusFilter = HealthStatus | 'all'
type EntityKindFilter = EntityKind | 'all'
type IssueSeverityFilter = IssueSeverity | 'all'
type ScanState = 'fixture' | 'cached' | 'scanning' | 'local' | 'error'

const statusFilters: StatusFilter[] = [
  'all',
  'ok',
  'needs-review',
  'at-risk',
  'broken',
]

const entityKindFilters: EntityKindFilter[] = [
  'all',
  'skill',
  'command',
  'agent',
  'plugin',
  'mcp-server',
  'instruction-file',
]

const issueSeverityFilters: IssueSeverityFilter[] = [
  'all',
  'info',
  'low',
  'medium',
  'high',
]

const navKeyByView: Record<ViewKey, keyof typeof appCopy['zh-CN']['nav']> = {
  overview: 'overview',
  extensions: 'extensions',
  mcp: 'mcpServers',
  plugins: 'plugins',
  sources: 'sources',
  issues: 'issues',
  settings: 'settings',
}

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

function formatReportDate(value: string, locale: Locale) {
  const date = new Date(value)

  if (Number.isNaN(date.getTime())) {
    return value
  }

  return new Intl.DateTimeFormat(locale, {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(date)
}

function formatReportSource(
  scanState: ScanState,
  copy: typeof appCopy['zh-CN'],
) {
  const labels: Record<ScanState, string> = {
    fixture: copy.dashboard.reportSourceFixture,
    cached: copy.dashboard.reportSourceCached,
    scanning: copy.dashboard.reportSourceScanning,
    local: copy.dashboard.reportSourceLocal,
    error: copy.dashboard.reportSourceError,
  }

  return labels[scanState]
}

function App() {
  const [locale, setLocale] = useState<Locale>(() => loadLocale())
  const [initialCachedReport] = useState(() => loadCachedReport())
  const [activeView, setActiveView] = useState<ViewKey>('overview')
  const [report, setReport] = useState<ScanReport>(
    () => initialCachedReport ?? fixtureScanReport,
  )
  const [scanState, setScanState] = useState<ScanState>(
    () => (initialCachedReport ? 'cached' : 'fixture'),
  )
  const [scanError, setScanError] = useState('')
  const [exportMessage, setExportMessage] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>('all')
  const [entityKindFilter, setEntityKindFilter] =
    useState<EntityKindFilter>('all')
  const [extensionQuery, setExtensionQuery] = useState('')
  const [issueSeverityFilter, setIssueSeverityFilter] =
    useState<IssueSeverityFilter>('all')
  const [issueQuery, setIssueQuery] = useState('')
  const [selectedEntityId, setSelectedEntityId] = useState(
    (initialCachedReport ?? fixtureScanReport).entities[0]?.id ?? '',
  )
  const copy = appCopy[locale]
  const totals = report.totals
  const selectedEntity =
    report.entities.find((entity) => entity.id === selectedEntityId) ??
    report.entities[0]

  async function scanLocalRoots() {
    setScanState('scanning')
    setScanError('')

    try {
      const result = await invoke('scan_local_extensions')
      const parsedReport = scanReportSchema.parse(result)
      setReport(parsedReport)
      saveCachedReport(parsedReport)
      setSelectedEntityId(parsedReport.entities[0]?.id ?? '')
      setScanState('local')
    } catch (error) {
      setScanError(error instanceof Error ? error.message : String(error))
      setScanState('error')
    }
  }

  async function exportCurrentReport() {
    setExportMessage('')

    try {
      const parsedReport = scanReportSchema.parse(report)
      const exportPath = await invoke<string>('export_scan_report', {
        report: parsedReport,
      })
      setExportMessage(`${copy.dashboard.exportSuccess}: ${exportPath}`)
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error)
      setExportMessage(`${copy.dashboard.exportError}: ${message}`)
    }
  }

  return (
    <main className="app-shell">
      <aside className="sidebar" aria-label="Primary navigation">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true">
            SD
          </div>
          <div>
            <h1>SkillDesk</h1>
            <p>{copy.brandSubtitle}</p>
          </div>
        </div>
        <nav>
          {viewOrder.map((view) => (
            <button
              key={view}
              type="button"
              aria-current={activeView === view ? 'page' : undefined}
              className="nav-button"
              onClick={() => setActiveView(view)}
            >
              {copy.nav[navKeyByView[view]]}
            </button>
          ))}
        </nav>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <p className="eyebrow">{copy.dashboard.eyebrow}</p>
            <h2>{copy.dashboard.title}</h2>
          </div>
          <div className="topbar-actions">
            <button
              type="button"
              className="secondary-button"
              onClick={() => {
                const next = nextLocale(locale)
                setLocale(next)
                saveLocale(next)
              }}
            >
              {copy.dashboard.languageButton}
            </button>
            <button
              type="button"
              disabled={scanState === 'scanning'}
              onClick={scanLocalRoots}
            >
              {scanState === 'scanning'
                ? copy.dashboard.scanningButton
                : copy.dashboard.scanButton}
            </button>
            <button
              type="button"
              className="secondary-button"
              disabled={scanState === 'scanning'}
              onClick={exportCurrentReport}
            >
              {copy.dashboard.exportButton}
            </button>
          </div>
        </header>
        {exportMessage && <p className="topbar-message">{exportMessage}</p>}

        {activeView === 'overview' && (
          <OverviewView
            copy={copy}
            locale={locale}
            report={report}
            totals={totals}
            scanState={scanState}
            scanError={scanError}
          />
        )}
        {activeView === 'extensions' && (
          <ExtensionsView
            copy={copy}
            entities={report.entities}
            statusFilter={statusFilter}
            onStatusFilterChange={setStatusFilter}
            kindFilter={entityKindFilter}
            onKindFilterChange={setEntityKindFilter}
            query={extensionQuery}
            onQueryChange={setExtensionQuery}
            selectedEntityId={selectedEntity?.id}
            onSelect={setSelectedEntityId}
          />
        )}
        {activeView === 'mcp' && (
          <McpView copy={copy} entities={report.entities} />
        )}
        {activeView === 'plugins' && (
          <PluginsView copy={copy} entities={report.entities} />
        )}
        {activeView === 'sources' && <SourcesView copy={copy} report={report} />}
        {activeView === 'issues' && (
          <IssuesView
            copy={copy}
            report={report}
            severityFilter={issueSeverityFilter}
            onSeverityFilterChange={setIssueSeverityFilter}
            query={issueQuery}
            onQueryChange={setIssueQuery}
          />
        )}
        {activeView === 'settings' && (
          <SettingsView
            copy={copy}
            locale={locale}
            settings={defaultAppSettings}
          />
        )}
      </section>
    </main>
  )
}

function OverviewView({
  copy,
  locale,
  report,
  totals,
  scanState,
  scanError,
}: {
  copy: typeof appCopy['zh-CN']
  locale: Locale
  report: ScanReport
  totals: ScanReport['totals']
  scanState: ScanState
  scanError: string
}) {
  const isLocalReport = scanState === 'local' || scanState === 'cached'

  return (
    <>
      <section className="summary-grid" aria-label="Health summary">
        <SummaryCard label={copy.dashboard.summary.totalExtensions} value={totals.entities} />
        <SummaryCard label={copy.dashboard.summary.needsReview} value={totals.byStatus['needs-review']} />
        <SummaryCard label={copy.dashboard.summary.mcpServers} value={totals.mcpServers} />
        <SummaryCard label={copy.dashboard.summary.broken} value={totals.byStatus.broken} />
        <SummaryCard label={copy.dashboard.summary.skills} value={totals.skills} />
        <SummaryCard label={copy.dashboard.summary.plugins} value={totals.plugins} />
      </section>

      <section className="panel">
        <div>
          <h3>
            {isLocalReport
              ? copy.dashboard.localPanelTitle
              : copy.dashboard.panelTitle}
          </h3>
          <p>
            {isLocalReport
              ? copy.dashboard.localPanelBody
              : copy.dashboard.panelBody}
          </p>
          {scanState === 'error' && <p className="error-text">{scanError}</p>}
        </div>
        <code>
          {isLocalReport
            ? copy.dashboard.localPhaseTag
            : copy.dashboard.phaseTag}
        </code>
      </section>

      <section className="report-meta" aria-label={copy.dashboard.reportSummaryTitle}>
        <ReportMetaItem
          label={copy.dashboard.reportSource}
          value={formatReportSource(scanState, copy)}
        />
        <ReportMetaItem
          label={copy.dashboard.generatedAt}
          value={formatReportDate(report.generatedAt, locale)}
        />
        <ReportMetaItem
          label={copy.dashboard.scanRoots}
          value={String(report.roots.length)}
        />
        <ReportMetaItem
          label={copy.dashboard.machinePlatform}
          value={report.machine.platform}
        />
        <ReportMetaItem
          label={copy.dashboard.schemaVersion}
          value={report.schemaVersion}
        />
      </section>

      <section className="split-grid">
        <EntityPreview copy={copy} entities={report.entities} />
        <IssuesPreview copy={copy} report={report} />
      </section>
    </>
  )
}

function SummaryCard({ label, value }: { label: string; value: number }) {
  return (
    <article>
      <span>{label}</span>
      <strong>{value}</strong>
    </article>
  )
}

function ReportMetaItem({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  )
}

function ExtensionsView({
  copy,
  entities,
  statusFilter,
  onStatusFilterChange,
  kindFilter,
  onKindFilterChange,
  query,
  onQueryChange,
  selectedEntityId,
  onSelect,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
  statusFilter: StatusFilter
  onStatusFilterChange: (status: StatusFilter) => void
  kindFilter: EntityKindFilter
  onKindFilterChange: (kind: EntityKindFilter) => void
  query: string
  onQueryChange: (query: string) => void
  selectedEntityId?: string
  onSelect: (id: string) => void
}) {
  const normalizedQuery = query.trim().toLowerCase()
  const filteredEntities = entities.filter((entity) => {
    const matchesStatus =
      statusFilter === 'all' || entity.health.status === statusFilter
    const matchesKind = kindFilter === 'all' || entity.kind === kindFilter
    const searchable = [
      entity.name,
      entity.title,
      entity.kind,
      entity.platform,
      entity.source,
      entity.path,
    ]
      .filter(Boolean)
      .join(' ')
      .toLowerCase()
    const matchesQuery =
      normalizedQuery.length === 0 || searchable.includes(normalizedQuery)

    return matchesStatus && matchesKind && matchesQuery
  })
  const selectedEntity =
    filteredEntities.find((entity) => entity.id === selectedEntityId) ??
    filteredEntities[0]
  const resultCount = copy.labels.resultCount
    .replace('{shown}', String(filteredEntities.length))
    .replace('{total}', String(entities.length))

  return (
    <section className="content-grid">
      <div className="table-panel">
        <SectionHeading title={copy.views.extensionsTitle} body={copy.views.extensionsBody} />
        <div className="table-tools">
          <label className="search-field">
            <span>{copy.labels.search}</span>
            <input
              type="search"
              value={query}
              placeholder={copy.labels.search}
              onChange={(event) => onQueryChange(event.target.value)}
            />
          </label>
          <span className="result-count">{resultCount}</span>
        </div>
        <StatusFilterBar
          copy={copy}
          activeStatus={statusFilter}
          onChange={onStatusFilterChange}
        />
        <EntityKindFilterBar
          copy={copy}
          activeKind={kindFilter}
          onChange={onKindFilterChange}
        />
        {filteredEntities.length > 0 ? (
          <EntityTable
            copy={copy}
            entities={filteredEntities}
            onSelect={onSelect}
            selectedEntityId={selectedEntity?.id}
          />
        ) : (
          <EmptyState message={copy.labels.emptyExtensions} />
        )}
      </div>
      {selectedEntity && <DetailPanel copy={copy} entity={selectedEntity} />}
    </section>
  )
}

function McpView({
  copy,
  entities,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
}) {
  const servers = entities.filter((entity) => entity.kind === 'mcp-server')

  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.mcpTitle} body={copy.views.readOnlyNotice} />
      {servers.length > 0 ? (
        <table>
          <thead>
            <tr>
              <th>{copy.labels.name}</th>
              <th>{copy.labels.transport}</th>
              <th>{copy.labels.status}</th>
              <th>{copy.labels.tools}</th>
              <th>{copy.labels.path}</th>
            </tr>
          </thead>
          <tbody>
            {servers.map((server) => (
              <tr key={server.id}>
                <td>{server.name}</td>
                <td>{server.kind === 'mcp-server' ? server.transport : ''}</td>
                <td>
                  <StatusBadge copy={copy} status={server.health.status} />
                </td>
                <td>{server.kind === 'mcp-server' ? (server.probe?.toolsCount ?? '-') : '-'}</td>
                <td className="path-cell">{server.path}</td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <EmptyState message={copy.labels.emptyMcp} />
      )}
    </section>
  )
}

function PluginsView({
  copy,
  entities,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
}) {
  const plugins = entities.filter((entity) => entity.kind === 'plugin')

  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.pluginsTitle} body={copy.views.readOnlyNotice} />
      {plugins.length > 0 ? (
        <table>
          <thead>
            <tr>
              <th>{copy.labels.name}</th>
              <th>{copy.labels.source}</th>
              <th>Skills</th>
              <th>MCP</th>
              <th>{copy.labels.status}</th>
            </tr>
          </thead>
          <tbody>
            {plugins.map((plugin) => (
              <tr key={plugin.id}>
                <td>{plugin.name}</td>
                <td>{plugin.source}</td>
                <td>{plugin.kind === 'plugin' ? plugin.bundled.skills : '-'}</td>
                <td>{plugin.kind === 'plugin' ? plugin.bundled.mcpServers : '-'}</td>
                <td>
                  <StatusBadge copy={copy} status={plugin.health.status} />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <EmptyState message={copy.labels.emptyPlugins} />
      )}
    </section>
  )
}

function SourcesView({
  copy,
  report,
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
}) {
  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.sourcesTitle} body={copy.views.readOnlyNotice} />
      {report.roots.length > 0 ? (
        <table>
          <thead>
            <tr>
              <th>{copy.labels.roots}</th>
              <th>{copy.labels.kind}</th>
              <th>{copy.labels.status}</th>
              <th>{copy.labels.issues}</th>
            </tr>
          </thead>
          <tbody>
            {report.roots.map((root) => (
              <tr key={root.path}>
                <td className="path-cell">{root.path}</td>
                <td>{root.kind}</td>
                <td>{root.status}</td>
                <td>{root.reason ?? '-'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <EmptyState message={copy.labels.emptySources} />
      )}
    </section>
  )
}

function IssuesView({
  copy,
  report,
  severityFilter,
  onSeverityFilterChange,
  query,
  onQueryChange,
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
  severityFilter: IssueSeverityFilter
  onSeverityFilterChange: (severity: IssueSeverityFilter) => void
  query: string
  onQueryChange: (query: string) => void
}) {
  const normalizedQuery = query.trim().toLowerCase()
  const filteredIssues = report.issues.filter((issue) => {
    const matchesSeverity =
      severityFilter === 'all' || issue.severity === severityFilter
    const searchable = [
      issue.severity,
      issue.category,
      issue.message,
      issue.file,
      issue.recommendation,
      issue.evidence,
    ]
      .filter(Boolean)
      .join(' ')
      .toLowerCase()
    const matchesQuery =
      normalizedQuery.length === 0 || searchable.includes(normalizedQuery)

    return matchesSeverity && matchesQuery
  })
  const resultCount = copy.labels.resultCount
    .replace('{shown}', String(filteredIssues.length))
    .replace('{total}', String(report.issues.length))

  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.issuesTitle} body={copy.views.readOnlyNotice} />
      <div className="table-tools">
        <label className="search-field">
          <span>{copy.labels.searchIssues}</span>
          <input
            type="search"
            value={query}
            placeholder={copy.labels.searchIssues}
            onChange={(event) => onQueryChange(event.target.value)}
          />
        </label>
        <span className="result-count">{resultCount}</span>
      </div>
      <IssueSeverityFilterBar
        copy={copy}
        activeSeverity={severityFilter}
        onChange={onSeverityFilterChange}
      />
      {filteredIssues.length > 0 ? (
        <div className="issue-card-list">
          {filteredIssues.map((issue) => (
            <IssueCard key={issue.id} copy={copy} issue={issue} />
          ))}
        </div>
      ) : (
        <EmptyState
          message={
            report.issues.length > 0
              ? copy.labels.emptyFilteredIssues
              : copy.labels.emptyIssues
          }
        />
      )}
    </section>
  )
}

function SettingsView({
  copy,
  locale,
  settings,
}: {
  copy: typeof appCopy['zh-CN']
  locale: Locale
  settings: AppSettings
}) {
  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.settingsTitle} body={copy.views.settingsBody} />
      <dl className="settings-list">
        <div>
          <dt>{copy.views.languageSetting}</dt>
          <dd>{locale}</dd>
        </div>
        <div>
          <dt>{copy.views.pluginCacheMode}</dt>
          <dd>
            {settings.includePluginCaches
              ? copy.labels.enabled
              : copy.views.pluginCacheSummaryOnly}
          </dd>
        </div>
        <div>
          <dt>{copy.views.mcpProbePolicy}</dt>
          <dd>{copy.views.mcpProbeDisabled}</dd>
        </div>
        <div>
          <dt>{copy.views.defaultScanRoots}</dt>
          <dd>
            <ul className="settings-roots">
              {settings.scanRoots.map((root) => (
                <li key={root} className="path-cell">
                  {root}
                </li>
              ))}
            </ul>
          </dd>
        </div>
        <div>
          <dt>{copy.views.scanSafety}</dt>
          <dd>{copy.views.scanSafetyBody}</dd>
        </div>
      </dl>
    </section>
  )
}

function EntityPreview({
  copy,
  entities,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
}) {
  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.extensionsTitle} />
      <EntityTable copy={copy} entities={entities.slice(0, 4)} />
    </section>
  )
}

function IssuesPreview({
  copy,
  report,
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
}) {
  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.issuesTitle} />
      {report.issues.length > 0 ? (
        <div className="issue-card-list">
          {report.issues.slice(0, 4).map((issue) => (
            <IssueCard key={issue.id} copy={copy} issue={issue} compact />
          ))}
        </div>
      ) : (
        <EmptyState message={copy.labels.emptyIssues} />
      )}
    </section>
  )
}

function EntityTable({
  copy,
  entities,
  selectedEntityId,
  onSelect,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
  selectedEntityId?: string
  onSelect?: (id: string) => void
}) {
  return (
    <table>
      <thead>
        <tr>
          <th>{copy.labels.name}</th>
          <th>{copy.labels.kind}</th>
          <th>{copy.labels.platform}</th>
          <th>{copy.labels.status}</th>
          <th>{copy.labels.issues}</th>
        </tr>
      </thead>
      <tbody>
        {entities.map((entity) => (
          <tr
            key={entity.id}
            className={selectedEntityId === entity.id ? 'selected-row' : undefined}
            onClick={() => onSelect?.(entity.id)}
          >
            <td>{entity.title ?? entity.name}</td>
            <td>{formatKind(entity.kind, copy)}</td>
            <td>{entity.platform}</td>
            <td>
              <StatusBadge copy={copy} status={entity.health.status} />
            </td>
            <td>{entity.health.issues.length}</td>
          </tr>
        ))}
      </tbody>
    </table>
  )
}

function DetailPanel({
  copy,
  entity,
}: {
  copy: typeof appCopy['zh-CN']
  entity: ManagedEntity
}) {
  return (
    <aside className="detail-panel">
      <SectionHeading title={copy.views.detailTitle} />
      <h3>{entity.title ?? entity.name}</h3>
      <StatusBadge copy={copy} status={entity.health.status} />
      <dl>
        <div>
          <dt>{copy.labels.kind}</dt>
          <dd>{formatKind(entity.kind, copy)}</dd>
        </div>
        <div>
          <dt>{copy.labels.platform}</dt>
          <dd>{entity.platform}</dd>
        </div>
        <div>
          <dt>{copy.labels.source}</dt>
          <dd>{entity.source}</dd>
        </div>
        <div>
          <dt>{copy.labels.path}</dt>
          <dd className="path-cell">{entity.path}</dd>
        </div>
      </dl>
      {entity.git && (
        <>
          <SectionHeading title={copy.labels.git} />
          <dl>
            <div>
              <dt>{copy.labels.gitRoot}</dt>
              <dd className="path-cell">{entity.git.root}</dd>
            </div>
            <div>
              <dt>{copy.labels.branch}</dt>
              <dd>{entity.git.branch ?? (entity.git.detached ? 'detached' : '-')}</dd>
            </div>
            <div>
              <dt>{copy.labels.commit}</dt>
              <dd className="path-cell">{entity.git.commit ?? '-'}</dd>
            </div>
            <div>
              <dt>{copy.labels.remote}</dt>
              <dd className="path-cell">{entity.git.remoteUrl ?? '-'}</dd>
            </div>
            <div>
              <dt>{copy.labels.status}</dt>
              <dd>{entity.git.dirty ? copy.labels.dirty : copy.labels.clean}</dd>
            </div>
          </dl>
        </>
      )}
      {entity.health.issues.length > 0 ? (
        <div className="issue-card-list">
          {entity.health.issues.map((issue) => (
            <IssueCard key={issue.id} copy={copy} issue={issue} compact />
          ))}
        </div>
      ) : (
        <EmptyState message={copy.labels.emptyIssues} />
      )}
    </aside>
  )
}

function IssueCard({
  copy,
  issue,
  compact = false,
}: {
  copy: typeof appCopy['zh-CN']
  issue: HealthIssue
  compact?: boolean
}) {
  return (
    <article className={compact ? 'issue-card issue-card-compact' : 'issue-card'}>
      <header>
        <span className={`severity-badge severity-${issue.severity}`}>
          {formatSeverity(issue.severity, copy)}
        </span>
        <span>{issue.category}</span>
      </header>
      <p>{issue.message}</p>
      {issue.recommendation && (
        <dl>
          <div>
            <dt>{copy.labels.recommendation}</dt>
            <dd>{issue.recommendation}</dd>
          </div>
        </dl>
      )}
      {!compact && issue.evidence && (
        <dl>
          <div>
            <dt>{copy.labels.evidence}</dt>
            <dd className="path-cell">{issue.evidence}</dd>
          </div>
        </dl>
      )}
      {!compact && issue.file && (
        <dl>
          <div>
            <dt>{copy.labels.path}</dt>
            <dd className="path-cell">{issue.file}</dd>
          </div>
        </dl>
      )}
    </article>
  )
}

function StatusBadge({
  copy,
  status,
}: {
  copy: typeof appCopy['zh-CN']
  status: HealthStatus
}) {
  return <span className={`status-badge status-${status}`}>{formatStatus(status, copy)}</span>
}

function StatusFilterBar({
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

function EntityKindFilterBar({
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

function IssueSeverityFilterBar({
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

function EmptyState({ message }: { message: string }) {
  return <p className="empty-state">{message}</p>
}

function SectionHeading({ title, body }: { title: string; body?: string }) {
  return (
    <header className="section-heading">
      <h3>{title}</h3>
      {body && <p>{body}</p>}
    </header>
  )
}

export default App
