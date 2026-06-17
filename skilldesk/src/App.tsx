import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

import {
  clearCachedReport,
  loadAppSettings,
  loadCachedReport,
  loadLocale,
  loadViewPreferences,
  saveAppSettings,
  saveCachedReport,
  saveLocale,
  saveViewPreferences,
} from './app/cache'
import { buildExportPayload, buildMarkdownReport } from './app/exportReport'
import { appCopy, nextLocale } from './app/i18n'
import './App.css'
import {
  ExtensionsView,
  IssuesView,
  McpView,
  OverviewView,
  PluginsView,
  SettingsView,
  SourcesView,
} from './features/views'
import {
  navKeyByView,
  viewOrder,
  type EntityKindFilter,
  type ExtensionSortKey,
  type IssueSeverityFilter,
  type ScanState,
  type SortDirection,
  type StatusFilter,
  type ViewKey,
  type ViewPreferences,
} from './features/viewState'
import { fixtureScanReport } from './fixtures'
import { scanReportSchema } from './model'
import type { AppSettings, Locale, ScanReport } from './model'

type ViewPreferencesPatch = {
  extensions?: Partial<ViewPreferences['extensions']>
  issues?: Partial<ViewPreferences['issues']>
}

function App({ initialView = 'overview' }: { initialView?: ViewKey } = {}) {
  const [locale, setLocale] = useState<Locale>(() => loadLocale())
  const [initialCachedReport] = useState(() => loadCachedReport())
  const [activeView, setActiveView] = useState<ViewKey>(initialView)
  const [report, setReport] = useState<ScanReport>(
    () => initialCachedReport ?? fixtureScanReport,
  )
  const [scanState, setScanState] = useState<ScanState>(
    () => (initialCachedReport ? 'cached' : 'fixture'),
  )
  const [viewPreferences, setViewPreferences] = useState<ViewPreferences>(() =>
    loadViewPreferences(),
  )
  const [scanError, setScanError] = useState('')
  const [exportMessage, setExportMessage] = useState('')
  const [statusFilter, setStatusFilter] = useState<StatusFilter>(
    () => viewPreferences.extensions.statusFilter,
  )
  const [entityKindFilter, setEntityKindFilter] =
    useState<EntityKindFilter>(() => viewPreferences.extensions.kindFilter)
  const [extensionQuery, setExtensionQuery] = useState(
    () => viewPreferences.extensions.query,
  )
  const [issueSeverityFilter, setIssueSeverityFilter] =
    useState<IssueSeverityFilter>(() => viewPreferences.issues.severityFilter)
  const [issueQuery, setIssueQuery] = useState(
    () => viewPreferences.issues.query,
  )
  const [settings, setSettings] = useState<AppSettings>(() => ({
    ...loadAppSettings(),
    locale,
  }))
  const [selectedEntityId, setSelectedEntityId] = useState(
    (initialCachedReport ?? fixtureScanReport).entities[0]?.id ?? '',
  )
  const copy = appCopy[locale]
  const totals = report.totals
  const selectedEntity =
    report.entities.find((entity) => entity.id === selectedEntityId) ??
    report.entities[0]

  function updateViewPreferences(patch: ViewPreferencesPatch) {
    setViewPreferences((current) => {
      const nextPreferences = {
        ...current,
        ...patch,
        extensions: {
          ...current.extensions,
          ...patch.extensions,
        },
        issues: {
          ...current.issues,
          ...patch.issues,
        },
      }
      saveViewPreferences(nextPreferences)
      return nextPreferences
    })
  }

  function updateStatusFilter(nextStatus: StatusFilter) {
    setStatusFilter(nextStatus)
    updateViewPreferences({ extensions: { statusFilter: nextStatus } })
  }

  function updateEntityKindFilter(nextKind: EntityKindFilter) {
    setEntityKindFilter(nextKind)
    updateViewPreferences({ extensions: { kindFilter: nextKind } })
  }

  function updateExtensionQuery(nextQuery: string) {
    setExtensionQuery(nextQuery)
    updateViewPreferences({ extensions: { query: nextQuery } })
  }

  function updateExtensionSort(
    sortKey: ExtensionSortKey,
    sortDirection: SortDirection,
  ) {
    updateViewPreferences({ extensions: { sortKey, sortDirection } })
  }

  function updateIssueSeverityFilter(nextSeverity: IssueSeverityFilter) {
    setIssueSeverityFilter(nextSeverity)
    updateViewPreferences({ issues: { severityFilter: nextSeverity } })
  }

  function updateIssueQuery(nextQuery: string) {
    setIssueQuery(nextQuery)
    updateViewPreferences({ issues: { query: nextQuery } })
  }

  function openEntityFromIssue(entityId: string) {
    setSelectedEntityId(entityId)
    setActiveView('extensions')
  }

  function toggleLocale() {
    const next = nextLocale(locale)
    setLocale(next)
    saveLocale(next)
    updateSettings({ locale: next })
  }

  function updateSettings(patch: Partial<AppSettings>) {
    setSettings((current) => {
      const nextSettings = { ...current, ...patch }
      saveAppSettings(nextSettings)
      return nextSettings
    })
  }

  async function scanLocalRoots() {
    setScanState('scanning')
    setScanError('')

    try {
      const result = await invoke('scan_local_extensions', {
        options: {
          includePluginCaches: settings.includePluginCaches,
          mcpProbePolicy: settings.mcpProbePolicy,
          scanRoots: settings.scanRoots,
        },
      })
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

  function currentExportPayload() {
    const parsedReport = scanReportSchema.parse(report)
    return buildExportPayload({
      exportedAt: new Date().toISOString(),
      report: parsedReport,
      reportSource: scanState,
      settings: {
        ...settings,
        locale,
      },
    })
  }

  async function exportCurrentReport() {
    setExportMessage('')

    try {
      const exportPayload = currentExportPayload()
      const exportPath = await invoke<string>('export_scan_report', {
        report: exportPayload,
      })
      setExportMessage(`${copy.dashboard.exportSuccess}: ${exportPath}`)
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error)
      setExportMessage(`${copy.dashboard.exportError}: ${message}`)
    }
  }

  async function exportCurrentMarkdownReport() {
    setExportMessage('')

    try {
      const exportPayload = currentExportPayload()
      const exportPath = await invoke<string>('export_markdown_report', {
        markdown: buildMarkdownReport(exportPayload),
        generatedAt: exportPayload.reportSummary.generatedAt,
      })
      setExportMessage(`${copy.dashboard.exportSuccess}: ${exportPath}`)
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error)
      setExportMessage(`${copy.dashboard.exportError}: ${message}`)
    }
  }

  function clearReportCache() {
    clearCachedReport()
    setReport(fixtureScanReport)
    setSelectedEntityId(fixtureScanReport.entities[0]?.id ?? '')
    setScanState('fixture')
    setScanError('')
    setExportMessage(copy.dashboard.cacheCleared)
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
        <div className="sidebar-note">
          <span>{copy.views.readOnlyScanning}</span>
          <p>{copy.views.readOnlyNotice}</p>
        </div>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <p className="eyebrow">{copy.dashboard.eyebrow}</p>
            <h2>{copy.dashboard.title}</h2>
          </div>
          <div className="topbar-actions">
            <span className={`scan-state scan-state-${scanState}`}>
              {scanState === 'local' || scanState === 'cached'
                ? copy.dashboard.localPhaseTag
                : copy.dashboard.phaseTag}
            </span>
            <button
              type="button"
              className="secondary-button"
              onClick={toggleLocale}
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
              {copy.dashboard.exportJsonButton}
            </button>
            <button
              type="button"
              className="secondary-button"
              disabled={scanState === 'scanning'}
              onClick={exportCurrentMarkdownReport}
            >
              {copy.dashboard.exportMarkdownButton}
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
            onStatusFilterChange={updateStatusFilter}
            kindFilter={entityKindFilter}
            onKindFilterChange={updateEntityKindFilter}
            query={extensionQuery}
            onQueryChange={updateExtensionQuery}
            sortKey={viewPreferences.extensions.sortKey}
            sortDirection={viewPreferences.extensions.sortDirection}
            onSortChange={updateExtensionSort}
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
        {activeView === 'sources' && (
          <SourcesView copy={copy} report={report} settings={settings} />
        )}
        {activeView === 'issues' && (
          <IssuesView
            copy={copy}
            report={report}
            severityFilter={issueSeverityFilter}
            onSeverityFilterChange={updateIssueSeverityFilter}
            query={issueQuery}
            onQueryChange={updateIssueQuery}
            onOpenEntity={openEntityFromIssue}
          />
        )}
        {activeView === 'settings' && (
          <SettingsView
            copy={copy}
            locale={locale}
            settings={settings}
            onIncludePluginCachesChange={(includePluginCaches) =>
              updateSettings({ includePluginCaches })
            }
            onScanRootsChange={(scanRoots) => updateSettings({ scanRoots })}
            onMcpProbePolicyChange={(mcpProbePolicy) =>
              updateSettings({ mcpProbePolicy })
            }
            onLocaleToggle={toggleLocale}
            onClearCache={clearReportCache}
          />
        )}
      </section>
    </main>
  )
}

export default App
