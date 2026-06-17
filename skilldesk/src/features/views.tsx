import { useMemo } from 'react'

import { appCopy } from '../app/i18n'
import {
  EmptyState,
  EntityKindFilterBar,
  IssueSeverityFilterBar,
  SectionHeading,
  StatusBadge,
  StatusFilterBar,
} from '../components/ui'
import type {
  AppSettings,
  EntityKind,
  HealthIssue,
  IssueSeverity,
  Locale,
  ManagedEntity,
  ScanReport,
} from '../model'

import {
  type ExtensionSortKey,
  type EntityKindFilter,
  type IssueSeverityFilter,
  type ScanState,
  type SortDirection,
  type StatusFilter,
} from './viewState'

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

function formatMcpProbePolicy(
  policy: AppSettings['mcpProbePolicy'],
  copy: typeof appCopy['zh-CN'],
) {
  const labels: Record<AppSettings['mcpProbePolicy'], string> = {
    disabled: copy.views.mcpProbeDisabled,
    'local-only': copy.views.mcpProbeLocalOnly,
    all: copy.views.mcpProbeAll,
  }

  return labels[policy]
}

export function OverviewView({
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
  const mojibakeIssueCount = useMemo(
    () => report.issues.filter((issue) => issue.category === 'encoding').length,
    [report.issues],
  )
  const dirtyGitSourceCount = useMemo(
    () => new Set(
      report.entities
        .filter((entity) => entity.git?.dirty)
        .map((entity) => entity.git?.root),
    ).size,
    [report.entities],
  )

  return (
    <>
      <section className="summary-grid" aria-label="Health summary">
        <SummaryCard label={copy.dashboard.summary.totalExtensions} value={totals.entities} />
        <SummaryCard label={copy.dashboard.summary.needsReview} value={totals.byStatus['needs-review']} />
        <SummaryCard label={copy.dashboard.summary.mcpServers} value={totals.mcpServers} />
        <SummaryCard label={copy.dashboard.summary.broken} value={totals.byStatus.broken} />
        <SummaryCard label={copy.dashboard.summary.skills} value={totals.skills} />
        <SummaryCard label={copy.dashboard.summary.commands} value={totals.commands} />
        <SummaryCard label={copy.dashboard.summary.agents} value={totals.agents} />
        <SummaryCard label={copy.dashboard.summary.plugins} value={totals.plugins} />
        <SummaryCard label={copy.dashboard.summary.mojibake} value={mojibakeIssueCount} />
        <SummaryCard label={copy.dashboard.summary.dirtyGitSources} value={dirtyGitSourceCount} />
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

export function ExtensionsView({
  copy,
  entities,
  statusFilter,
  onStatusFilterChange,
  kindFilter,
  onKindFilterChange,
  query,
  onQueryChange,
  sortKey,
  sortDirection,
  onSortChange,
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
  sortKey: ExtensionSortKey
  sortDirection: SortDirection
  onSortChange: (sortKey: ExtensionSortKey, sortDirection: SortDirection) => void
  selectedEntityId?: string
  onSelect: (id: string) => void
}) {
  const normalizedQuery = query.trim().toLowerCase()
  const filteredEntities = useMemo(() => {
    const matchingEntities = entities.filter((entity) => {
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

    return sortEntities(matchingEntities, sortKey, sortDirection)
  }, [entities, kindFilter, normalizedQuery, sortDirection, sortKey, statusFilter])
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
            sortKey={sortKey}
            sortDirection={sortDirection}
            onSortChange={onSortChange}
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

export function McpView({
  copy,
  entities,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
}) {
  const servers = useMemo(
    () => entities.filter((entity) => entity.kind === 'mcp-server'),
    [entities],
  )

  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.mcpTitle} body={copy.views.readOnlyNotice} />
      {servers.length > 0 ? (
        <div className="mcp-card-list">
          {servers.map((server) => (
            <article key={server.id} className="mcp-card">
              <header>
                <div>
                  <h4>{server.title ?? server.name}</h4>
                  <span>{server.transport}</span>
                </div>
                <StatusBadge copy={copy} status={server.health.status} />
              </header>
              <dl className="mcp-meta">
                <div>
                  <dt>{copy.labels.command}</dt>
                  <dd className="path-cell">{server.command ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.args}</dt>
                  <dd>{server.argsCount ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.host}</dt>
                  <dd>{server.urlHost ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.tools}</dt>
                  <dd>{server.probe?.toolsCount ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.resources}</dt>
                  <dd>{server.probe?.resourcesCount ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.prompts}</dt>
                  <dd>{server.probe?.promptsCount ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.probe}</dt>
                  <dd>
                    {server.probe?.attempted
                      ? copy.labels.probeAttempted
                      : copy.labels.probeSkipped}
                  </dd>
                </div>
                <div>
                  <dt>{copy.labels.path}</dt>
                  <dd className="path-cell">{server.configPath}</dd>
                </div>
              </dl>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState message={copy.labels.emptyMcp} />
      )}
    </section>
  )
}

export function PluginsView({
  copy,
  entities,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
}) {
  const plugins = useMemo(
    () => entities.filter((entity) => entity.kind === 'plugin'),
    [entities],
  )

  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.pluginsTitle} body={copy.views.readOnlyNotice} />
      {plugins.length > 0 ? (
        <div className="plugin-card-list">
          {plugins.map((plugin) => (
            <article key={plugin.id} className="plugin-card">
              <header>
                <div>
                  <h4>{plugin.title ?? plugin.name}</h4>
                  <span>{plugin.source}</span>
                </div>
                <StatusBadge copy={copy} status={plugin.health.status} />
              </header>
              <dl className="plugin-meta">
                <div>
                  <dt>{copy.labels.version}</dt>
                  <dd>{plugin.version ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.publisher}</dt>
                  <dd>{plugin.publisher ?? '-'}</dd>
                </div>
                <div>
                  <dt>{copy.labels.cache}</dt>
                  <dd>{plugin.cache.isCache ? copy.labels.yes : copy.labels.no}</dd>
                </div>
                <div>
                  <dt>{copy.labels.backup}</dt>
                  <dd>{plugin.cache.isBackup ? copy.labels.yes : copy.labels.no}</dd>
                </div>
                <div>
                  <dt>Skills</dt>
                  <dd>{plugin.bundled.skills}</dd>
                </div>
                <div>
                  <dt>{copy.labels.commands}</dt>
                  <dd>{plugin.bundled.commands}</dd>
                </div>
                <div>
                  <dt>{copy.labels.agents}</dt>
                  <dd>{plugin.bundled.agents}</dd>
                </div>
                <div>
                  <dt>MCP</dt>
                  <dd>{plugin.bundled.mcpServers}</dd>
                </div>
                <div>
                  <dt>Hooks</dt>
                  <dd>{plugin.bundled.hooks}</dd>
                </div>
                <div>
                  <dt>{copy.labels.manifest}</dt>
                  <dd className="path-cell">{plugin.manifestPath ?? '-'}</dd>
                </div>
              </dl>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState message={copy.labels.emptyPlugins} />
      )}
    </section>
  )
}

export function SourcesView({
  copy,
  report,
  settings,
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
  settings: AppSettings
}) {
  const instructionFiles = useMemo(
    () => report.entities.filter((entity) => entity.kind === 'instruction-file'),
    [report.entities],
  )
  const scannedRoots = useMemo(
    () => report.roots.filter((root) => root.status === 'scanned'),
    [report.roots],
  )
  const missingRoots = useMemo(
    () => report.roots.filter((root) => root.status === 'missing'),
    [report.roots],
  )
  const skippedRoots = useMemo(
    () => report.roots.filter((root) => root.status === 'skipped'),
    [report.roots],
  )
  const errorRoots = useMemo(
    () => report.roots.filter((root) => root.status === 'error'),
    [report.roots],
  )

  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.sourcesTitle} body={copy.views.readOnlyNotice} />
      <section className="source-health-grid" aria-label={copy.views.scannerSelfCheck}>
        <SourceHealthItem label={copy.labels.scanned} value={scannedRoots.length} />
        <SourceHealthItem label={copy.labels.missing} value={missingRoots.length} />
        <SourceHealthItem label={copy.labels.skipped} value={skippedRoots.length} />
        <SourceHealthItem label={copy.labels.errors} value={errorRoots.length} />
      </section>
      <div className="capability-list" aria-label={copy.views.scannerCapabilities}>
        <CapabilityItem label={copy.views.readOnlyScanning} value={copy.labels.enabled} />
        <CapabilityItem
          label={copy.views.pluginCacheMode}
          value={
            settings.includePluginCaches
              ? copy.labels.enabled
              : copy.views.pluginCacheSummaryOnly
          }
        />
        <CapabilityItem
          label={copy.views.mcpProbePolicy}
          value={formatMcpProbePolicy(settings.mcpProbePolicy, copy)}
        />
      </div>
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
      <SectionHeading title={copy.views.instructionFilesTitle} />
      {instructionFiles.length > 0 ? (
        <div className="instruction-card-list">
          {instructionFiles.map((file) => (
            <article key={file.id} className="instruction-card">
              <header>
                <div>
                  <h4>{file.title ?? file.name}</h4>
                  <span>{file.fileType}</span>
                </div>
                <StatusBadge copy={copy} status={file.health.status} />
              </header>
              <dl className="instruction-meta">
                <div>
                  <dt>{copy.labels.appliesTo}</dt>
                  <dd className="path-cell">{file.appliesToPath}</dd>
                </div>
                <div>
                  <dt>{copy.labels.lines}</dt>
                  <dd>{file.lineCount}</dd>
                </div>
                <div>
                  <dt>{copy.labels.path}</dt>
                  <dd className="path-cell">{file.path}</dd>
                </div>
              </dl>
            </article>
          ))}
        </div>
      ) : (
        <EmptyState message={copy.labels.emptyInstructionFiles} />
      )}
    </section>
  )
}

function SourceHealthItem({ label, value }: { label: string; value: number }) {
  return (
    <article>
      <span>{label}</span>
      <strong>{value}</strong>
    </article>
  )
}

function CapabilityItem({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  )
}

export function IssuesView({
  copy,
  report,
  severityFilter,
  onSeverityFilterChange,
  query,
  onQueryChange,
  onOpenEntity,
}: {
  copy: typeof appCopy['zh-CN']
  report: ScanReport
  severityFilter: IssueSeverityFilter
  onSeverityFilterChange: (severity: IssueSeverityFilter) => void
  query: string
  onQueryChange: (query: string) => void
  onOpenEntity: (entityId: string) => void
}) {
  const normalizedQuery = query.trim().toLowerCase()
  const filteredIssues = useMemo(
    () =>
      report.issues.filter((issue) => {
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
      }),
    [normalizedQuery, report.issues, severityFilter],
  )
  const resultCount = copy.labels.resultCount
    .replace('{shown}', String(filteredIssues.length))
    .replace('{total}', String(report.issues.length))
  const entityByIssueId = useMemo(
    () => buildIssueEntityMap(report.entities),
    [report.entities],
  )

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
          {filteredIssues.map((issue) => {
            const owningEntity = findIssueEntity(
              issue,
              report.entities,
              entityByIssueId,
            )

            return (
              <IssueCard
                key={issue.id}
                copy={copy}
                issue={issue}
                entity={owningEntity}
                onOpenEntity={owningEntity ? onOpenEntity : undefined}
              />
            )
          })}
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

export function SettingsView({
  copy,
  locale,
  settings,
  onIncludePluginCachesChange,
  onScanRootsChange,
  onMcpProbePolicyChange,
  onLocaleToggle,
  onClearCache,
}: {
  copy: typeof appCopy['zh-CN']
  locale: Locale
  settings: AppSettings
  onIncludePluginCachesChange: (includePluginCaches: boolean) => void
  onScanRootsChange: (scanRoots: string[]) => void
  onMcpProbePolicyChange: (mcpProbePolicy: AppSettings['mcpProbePolicy']) => void
  onLocaleToggle: () => void
  onClearCache: () => void
}) {
  return (
    <section className="table-panel">
      <SectionHeading title={copy.views.settingsTitle} body={copy.views.settingsBody} />
      <dl className="settings-list">
        <div>
          <dt>{copy.views.languageSetting}</dt>
          <dd className="setting-inline">
            <span>{locale}</span>
            <button type="button" className="secondary-button" onClick={onLocaleToggle}>
              {copy.dashboard.languageButton}
            </button>
          </dd>
        </div>
        <div>
          <dt>{copy.views.pluginCacheMode}</dt>
          <dd>
            <label className="toggle-field">
              <input
                type="checkbox"
                checked={settings.includePluginCaches}
                onChange={(event) =>
                  onIncludePluginCachesChange(event.currentTarget.checked)
                }
              />
              <span>
                {settings.includePluginCaches
                  ? copy.labels.enabled
                  : copy.views.pluginCacheSummaryOnly}
              </span>
            </label>
          </dd>
        </div>
        <div>
          <dt>{copy.views.mcpProbePolicy}</dt>
          <dd>
            <select
              value={settings.mcpProbePolicy}
              onChange={(event) =>
                onMcpProbePolicyChange(
                  event.currentTarget.value as AppSettings['mcpProbePolicy'],
                )
              }
            >
              <option value="disabled">{copy.views.mcpProbeDisabled}</option>
              <option value="local-only">{copy.views.mcpProbeLocalOnly}</option>
              <option value="all">{copy.views.mcpProbeAll}</option>
            </select>
          </dd>
        </div>
        <div>
          <dt>{copy.views.defaultScanRoots}</dt>
          <dd>
            <textarea
              className="settings-roots-editor"
              defaultValue={settings.scanRoots.join('\n')}
              rows={Math.max(6, settings.scanRoots.length)}
              spellCheck={false}
              onBlur={(event) => {
                const scanRoots = event.currentTarget.value
                  .split(/\r?\n/)
                  .map((root) => root.trim())
                  .filter(Boolean)
                onScanRootsChange(scanRoots)
              }}
            />
            <p className="settings-help">{copy.views.scanRootsHelp}</p>
          </dd>
        </div>
        <div>
          <dt>{copy.views.scanSafety}</dt>
          <dd>{copy.views.scanSafetyBody}</dd>
        </div>
      </dl>
      <div className="settings-actions">
        <button type="button" className="secondary-button" onClick={onClearCache}>
          {copy.dashboard.clearCacheButton}
        </button>
      </div>
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
  sortKey,
  sortDirection,
  onSortChange,
}: {
  copy: typeof appCopy['zh-CN']
  entities: ManagedEntity[]
  selectedEntityId?: string
  onSelect?: (id: string) => void
  sortKey?: ExtensionSortKey
  sortDirection?: SortDirection
  onSortChange?: (sortKey: ExtensionSortKey, sortDirection: SortDirection) => void
}) {
  function sortLabel(key: ExtensionSortKey, label: string) {
    if (sortKey !== key) {
      return label
    }

    return `${label} ${sortDirection === 'asc' ? '↑' : '↓'}`
  }

  function requestSort(key: ExtensionSortKey) {
    if (!onSortChange) {
      return
    }

    const nextDirection =
      sortKey === key && sortDirection === 'asc' ? 'desc' : 'asc'
    onSortChange(key, nextDirection)
  }

  function sortableHeader(key: ExtensionSortKey, label: string) {
    if (!onSortChange) {
      return label
    }

    return (
      <button
        type="button"
        className="table-sort-button"
        aria-pressed={sortKey === key}
        onClick={() => requestSort(key)}
      >
        {sortLabel(key, label)}
      </button>
    )
  }

  return (
    <table>
      <thead>
        <tr>
          <th>{sortableHeader('name', copy.labels.name)}</th>
          <th>{sortableHeader('kind', copy.labels.kind)}</th>
          <th>{sortableHeader('platform', copy.labels.platform)}</th>
          <th>{sortableHeader('health', copy.labels.status)}</th>
          <th>{sortableHeader('issues', copy.labels.issues)}</th>
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
      <EntityKindDetails copy={copy} entity={entity} />
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

function EntityKindDetails({
  copy,
  entity,
}: {
  copy: typeof appCopy['zh-CN']
  entity: ManagedEntity
}) {
  switch (entity.kind) {
    case 'skill':
      return (
        <dl className="entity-kind-details">
          <OptionalDetailRow
            label={copy.labels.declaredVersion}
            value={entity.declaredVersion}
          />
          <DetailRow label={copy.labels.skillFile} value={entity.files.skillMd} isPath />
          <DetailRow label="openai.yaml" value={entity.files.openaiYaml ?? '-'} isPath />
          <DetailRow label="CLAUDE.md" value={entity.files.claudeMd ?? '-'} isPath />
          <DetailRow label={copy.labels.scripts} value={entity.files.scripts.length} />
          <DetailRow label={copy.labels.references} value={entity.files.references.length} />
          <DetailRow label={copy.labels.assets} value={entity.files.assets.length} />
        </dl>
      )

    case 'command':
      return (
        <dl className="entity-kind-details">
          <DetailRow label={copy.labels.commandType} value={entity.commandType} />
          <DetailRow label={copy.labels.namespace} value={entity.namespace ?? '-'} />
          <DetailRow label={copy.labels.file} value={entity.file} isPath />
        </dl>
      )

    case 'agent':
      return (
        <dl className="entity-kind-details">
          <DetailRow label={copy.labels.model} value={entity.declaredModel ?? '-'} />
          <DetailRow
            label={copy.labels.declaredTools}
            value={entity.declaredTools.length > 0 ? entity.declaredTools.join(', ') : '-'}
          />
          <DetailRow label={copy.labels.file} value={entity.file} isPath />
        </dl>
      )

    case 'plugin':
      return (
        <dl className="entity-kind-details">
          <DetailRow label={copy.labels.version} value={entity.version ?? '-'} />
          <DetailRow label={copy.labels.publisher} value={entity.publisher ?? '-'} />
          <DetailRow label={copy.labels.manifest} value={entity.manifestPath ?? '-'} isPath />
          <DetailRow label={copy.labels.kindSkill} value={entity.bundled.skills} />
          <DetailRow label={copy.labels.commands} value={entity.bundled.commands} />
          <DetailRow label={copy.labels.agents} value={entity.bundled.agents} />
          <DetailRow label={copy.labels.mcpServers} value={entity.bundled.mcpServers} />
          <DetailRow label={copy.labels.hooks} value={entity.bundled.hooks} />
          <DetailRow label={copy.labels.cache} value={formatBoolean(entity.cache.isCache, copy)} />
          <DetailRow label={copy.labels.backup} value={formatBoolean(entity.cache.isBackup, copy)} />
          <DetailRow label={copy.labels.cacheFamily} value={entity.cache.cacheFamily ?? '-'} />
        </dl>
      )

    case 'mcp-server':
      return (
        <dl className="entity-kind-details">
          <DetailRow label={copy.labels.file} value={entity.configPath} isPath />
          <DetailRow label={copy.labels.transport} value={entity.transport} />
          <DetailRow label={copy.labels.command} value={entity.command ?? '-'} isPath />
          <DetailRow label={copy.labels.args} value={entity.argsCount ?? 0} />
          <DetailRow label={copy.labels.host} value={entity.urlHost ?? '-'} />
          <DetailRow
            label={copy.labels.probeAttempted}
            value={formatBoolean(entity.probe?.attempted ?? false, copy)}
          />
          <DetailRow
            label={copy.labels.reachable}
            value={
              entity.probe?.reachable === undefined
                ? copy.labels.probeSkipped
                : formatBoolean(entity.probe.reachable, copy)
            }
          />
          <DetailRow label={copy.labels.tools} value={entity.probe?.toolsCount ?? 0} />
          <DetailRow label={copy.labels.resources} value={entity.probe?.resourcesCount ?? 0} />
          <DetailRow label={copy.labels.prompts} value={entity.probe?.promptsCount ?? 0} />
          <DetailRow
            label={copy.labels.latency}
            value={entity.probe?.latencyMs ? `${entity.probe.latencyMs} ms` : '-'}
          />
          <DetailRow label={copy.labels.error} value={entity.probe?.error ?? '-'} />
        </dl>
      )

    case 'instruction-file':
      return (
        <dl className="entity-kind-details">
          <DetailRow label={copy.labels.file} value={entity.fileType} />
          <DetailRow label={copy.labels.appliesTo} value={entity.appliesToPath} isPath />
          <DetailRow label={copy.labels.lines} value={entity.lineCount} />
        </dl>
      )
  }
}

function DetailRow({
  label,
  value,
  isPath = false,
}: {
  label: string
  value: string | number
  isPath?: boolean
}) {
  return (
    <div>
      <dt>{label}</dt>
      <dd className={isPath ? 'path-cell' : undefined}>{value}</dd>
    </div>
  )
}

function OptionalDetailRow({
  label,
  value,
}: {
  label: string
  value?: string
}) {
  if (!value) {
    return null
  }

  return <DetailRow label={label} value={value} />
}

function formatBoolean(value: boolean, copy: typeof appCopy['zh-CN']) {
  return value ? copy.labels.yes : copy.labels.no
}

function IssueCard({
  copy,
  issue,
  compact = false,
  entity,
  onOpenEntity,
}: {
  copy: typeof appCopy['zh-CN']
  issue: HealthIssue
  compact?: boolean
  entity?: ManagedEntity
  onOpenEntity?: (entityId: string) => void
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
      {!compact && entity && onOpenEntity && (
        <button
          type="button"
          className="secondary-button issue-entity-button"
          onClick={() => onOpenEntity(entity.id)}
        >
          {entity.title ?? entity.name}
        </button>
      )}
    </article>
  )
}

function sortEntities(
  entities: ManagedEntity[],
  sortKey: ExtensionSortKey,
  sortDirection: SortDirection,
) {
  const direction = sortDirection === 'asc' ? 1 : -1

  return [...entities].sort((left, right) => {
    const comparison = compareEntities(left, right, sortKey)

    if (comparison !== 0) {
      return comparison * direction
    }

    return compareText(left.title ?? left.name, right.title ?? right.name)
  })
}

function compareEntities(
  left: ManagedEntity,
  right: ManagedEntity,
  sortKey: ExtensionSortKey,
) {
  switch (sortKey) {
    case 'health':
      return (
        healthStatusRank(left.health.status) -
          healthStatusRank(right.health.status) ||
        right.health.issues.length - left.health.issues.length ||
        compareText(left.kind, right.kind)
      )
    case 'name':
      return compareText(left.title ?? left.name, right.title ?? right.name)
    case 'kind':
      return compareText(left.kind, right.kind)
    case 'platform':
      return compareText(left.platform, right.platform)
    case 'issues':
      return left.health.issues.length - right.health.issues.length
  }
}

function healthStatusRank(status: ManagedEntity['health']['status']) {
  const ranks: Record<ManagedEntity['health']['status'], number> = {
    broken: 0,
    'at-risk': 1,
    'needs-review': 2,
    ok: 3,
  }

  return ranks[status]
}

function compareText(left: string, right: string) {
  return left.localeCompare(right, undefined, {
    numeric: true,
    sensitivity: 'base',
  })
}

function buildIssueEntityMap(entities: ManagedEntity[]) {
  const issueEntityMap = new Map<string, ManagedEntity>()

  for (const entity of entities) {
    for (const issue of entity.health.issues) {
      issueEntityMap.set(issue.id, entity)
    }
  }

  return issueEntityMap
}

function findIssueEntity(
  issue: HealthIssue,
  entities: ManagedEntity[],
  entityByIssueId: Map<string, ManagedEntity>,
) {
  const exactMatch = entityByIssueId.get(issue.id)
  if (exactMatch) {
    return exactMatch
  }

  const issuePath = issue.file ?? issue.evidence
  if (!issuePath) {
    return undefined
  }

  const normalizedIssuePath = normalizePath(issuePath)

  return entities.find((entity) => {
    const normalizedEntityPath = normalizePath(entity.path)
    return (
      normalizedIssuePath.startsWith(normalizedEntityPath) ||
      normalizedEntityPath.startsWith(normalizedIssuePath)
    )
  })
}

function normalizePath(path: string) {
  return path.replaceAll('\\', '/').toLowerCase()
}

