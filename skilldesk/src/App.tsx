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
import { scanReportSchema } from './model'
import type { EntityKind, HealthStatus, Locale, ManagedEntity, ScanReport } from './model'

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

const statusFilters: StatusFilter[] = [
  'all',
  'ok',
  'needs-review',
  'at-risk',
  'broken',
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

function formatKind(kind: EntityKind) {
  return kind.replace('-', ' ')
}

function App() {
  const [locale, setLocale] = useState<Locale>(() => loadLocale())
  const [initialCachedReport] = useState(() => loadCachedReport())
  const [activeView, setActiveView] = useState<ViewKey>('overview')
  const [report, setReport] = useState<ScanReport>(
    () => initialCachedReport ?? fixtureScanReport,
  )
  const [scanState, setScanState] = useState<'fixture' | 'scanning' | 'local' | 'error'>(
    () => (initialCachedReport ? 'local' : 'fixture'),
  )
  const [scanError, setScanError] = useState('')
  const [exportMessage, setExportMessage] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>('all')
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
        {activeView === 'issues' && <IssuesView copy={copy} report={report} />}
        {activeView === 'settings' && (
          <SettingsView copy={copy} locale={locale} />
        )}
      </section>
    </main>
  )
}

function OverviewView({
  copy,
  report,
  totals,
  scanState,
  scanError,
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
  totals: ScanReport['totals']
  scanState: 'fixture' | 'scanning' | 'local' | 'error'
  scanError: string
}) {
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
            {scanState === 'local'
              ? copy.dashboard.localPanelTitle
              : copy.dashboard.panelTitle}
          </h3>
          <p>
            {scanState === 'local'
              ? copy.dashboard.localPanelBody
              : copy.dashboard.panelBody}
          </p>
          {scanState === 'error' && <p className="error-text">{scanError}</p>}
        </div>
        <code>
          {scanState === 'local'
            ? copy.dashboard.localPhaseTag
            : copy.dashboard.phaseTag}
        </code>
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

function ExtensionsView({
  copy,
  entities,
  statusFilter,
  onStatusFilterChange,
  selectedEntityId,
  onSelect,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
  statusFilter: StatusFilter
  onStatusFilterChange: (status: StatusFilter) => void
  selectedEntityId?: string
  onSelect: (id: string) => void
}) {
  const filteredEntities =
    statusFilter === 'all'
      ? entities
      : entities.filter((entity) => entity.health.status === statusFilter)
  const selectedEntity =
    filteredEntities.find((entity) => entity.id === selectedEntityId) ??
    filteredEntities[0]

  return (
    <section className="content-grid">
      <div className="table-panel">
        <SectionHeading title={copy.views.extensionsTitle} body={copy.views.extensionsBody} />
        <StatusFilterBar
          copy={copy}
          activeStatus={statusFilter}
          onChange={onStatusFilterChange}
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
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
}) {
  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.issuesTitle} body={copy.views.readOnlyNotice} />
      {report.issues.length > 0 ? (
        <table>
          <thead>
            <tr>
              <th>{copy.labels.status}</th>
              <th>{copy.labels.kind}</th>
              <th>{copy.labels.issues}</th>
              <th>{copy.labels.path}</th>
            </tr>
          </thead>
          <tbody>
            {report.issues.map((issue) => (
              <tr key={issue.id}>
                <td>{issue.severity}</td>
                <td>{issue.category}</td>
                <td>{issue.message}</td>
                <td className="path-cell">{issue.file ?? '-'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <EmptyState message={copy.labels.emptyIssues} />
      )}
    </section>
  )
}

function SettingsView({
  copy,
  locale,
}: {
  copy: typeof appCopy['zh-CN']
  locale: Locale
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
          <dd>summary-only</dd>
        </div>
        <div>
          <dt>{copy.views.mcpProbePolicy}</dt>
          <dd>disabled</dd>
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
      <ul className="issue-list">
        {report.issues.length > 0 ? (
          report.issues.map((issue) => (
            <li key={issue.id}>
              <strong>{issue.severity}</strong>
              <span>{issue.message}</span>
            </li>
          ))
        ) : (
          <li>
            <span>{copy.labels.emptyIssues}</span>
          </li>
        )}
      </ul>
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
            <td>{formatKind(entity.kind)}</td>
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
          <dd>{formatKind(entity.kind)}</dd>
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
      <ul className="issue-list">
        {entity.health.issues.map((issue) => (
          <li key={issue.id}>
            <strong>{issue.severity}</strong>
            <span>{issue.recommendation ?? issue.message}</span>
          </li>
        ))}
      </ul>
    </aside>
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
