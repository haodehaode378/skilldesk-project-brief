import type { AppSettings, ScanReport } from '../model'

export type ReportSource = 'fixture' | 'cached' | 'scanning' | 'local' | 'error'

export type SkillDeskExportPayload = {
  exportSchemaVersion: '0.1'
  exportedAt: string
  reportSource: ReportSource
  reportSummary: {
    generatedAt: string
    schemaVersion: ScanReport['schemaVersion']
    machinePlatform: ScanReport['machine']['platform']
    roots: number
    entities: number
    issues: number
  }
  scannerSelfCheck: {
    scanned: number
    missing: number
    skipped: number
    errors: number
  }
  capabilities: {
    readOnlyScanning: true
    pluginCacheMode: 'summary-only' | 'deep'
    mcpProbePolicy: AppSettings['mcpProbePolicy']
  }
  scanReport: ScanReport
}

export function buildExportPayload({
  exportedAt,
  report,
  reportSource,
  settings,
}: {
  exportedAt: string
  report: ScanReport
  reportSource: ReportSource
  settings: AppSettings
}): SkillDeskExportPayload {
  return {
    exportSchemaVersion: '0.1',
    exportedAt,
    reportSource,
    reportSummary: {
      generatedAt: report.generatedAt,
      schemaVersion: report.schemaVersion,
      machinePlatform: report.machine.platform,
      roots: report.roots.length,
      entities: report.totals.entities,
      issues: report.issues.length,
    },
    scannerSelfCheck: {
      scanned: countRootsByStatus(report, 'scanned'),
      missing: countRootsByStatus(report, 'missing'),
      skipped: countRootsByStatus(report, 'skipped'),
      errors: countRootsByStatus(report, 'error'),
    },
    capabilities: {
      readOnlyScanning: true,
      pluginCacheMode: settings.includePluginCaches ? 'deep' : 'summary-only',
      mcpProbePolicy: settings.mcpProbePolicy,
    },
    scanReport: report,
  }
}

function countRootsByStatus(
  report: ScanReport,
  status: ScanReport['roots'][number]['status'],
) {
  return report.roots.filter((root) => root.status === status).length
}
