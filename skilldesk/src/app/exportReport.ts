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

export function buildMarkdownReport(payload: SkillDeskExportPayload): string {
  const report = payload.scanReport
  const lines = [
    '# SkillDesk 扫描报告',
    '',
    `- 导出时间: ${payload.exportedAt}`,
    `- 报告生成时间: ${payload.reportSummary.generatedAt}`,
    `- 数据来源: ${payload.reportSource}`,
    `- 平台: ${payload.reportSummary.machinePlatform}`,
    `- Schema: ${payload.reportSummary.schemaVersion}`,
    '',
    '## 摘要',
    '',
    `- 扫描来源: ${payload.reportSummary.roots}`,
    `- 扩展总数: ${payload.reportSummary.entities}`,
    `- 问题总数: ${payload.reportSummary.issues}`,
    `- 只读扫描: ${payload.capabilities.readOnlyScanning ? 'yes' : 'no'}`,
    `- 插件缓存策略: ${payload.capabilities.pluginCacheMode}`,
    `- MCP 探测策略: ${payload.capabilities.mcpProbePolicy}`,
    '',
    '## 健康状态',
    '',
    `- ok: ${report.totals.byStatus.ok}`,
    `- needs-review: ${report.totals.byStatus['needs-review']}`,
    `- at-risk: ${report.totals.byStatus['at-risk']}`,
    `- broken: ${report.totals.byStatus.broken}`,
    '',
    '## 类型统计',
    '',
    `- skills: ${report.totals.skills}`,
    `- commands: ${report.totals.commands}`,
    `- agents: ${report.totals.agents}`,
    `- plugins: ${report.totals.plugins}`,
    `- MCP servers: ${report.totals.mcpServers}`,
    `- instruction files: ${report.totals.instructionFiles}`,
    '',
    '## 扫描来源',
    '',
    ...report.roots.map(
      (root) =>
        `- ${escapeMarkdown(root.path)} (${root.kind}, ${root.status})${
          root.reason ? ` - ${escapeMarkdown(root.reason)}` : ''
        }`,
    ),
    '',
    '## 问题',
    '',
    ...formatIssues(payload),
    '',
    '## 扩展清单',
    '',
    ...report.entities.map(
      (entity) =>
        `- ${escapeMarkdown(entity.title ?? entity.name)} (${entity.kind}, ${entity.platform}, ${entity.health.status}) - ${escapeMarkdown(entity.path)}`,
    ),
    '',
  ]

  return lines.join('\n')
}

function formatIssues(payload: SkillDeskExportPayload): string[] {
  if (payload.scanReport.issues.length === 0) {
    return ['- 未发现问题。']
  }

  return payload.scanReport.issues.flatMap((issue) => [
    `- [${issue.severity}] ${escapeMarkdown(issue.message)}`,
    issue.file ? `  - path: ${escapeMarkdown(issue.file)}` : '',
    issue.evidence ? `  - evidence: ${escapeMarkdown(issue.evidence)}` : '',
    issue.recommendation
      ? `  - recommendation: ${escapeMarkdown(issue.recommendation)}`
      : '',
  ]).filter(Boolean)
}

function countRootsByStatus(
  report: ScanReport,
  status: ScanReport['roots'][number]['status'],
) {
  return report.roots.filter((root) => root.status === status).length
}

function escapeMarkdown(value: string) {
  return value.replace(/\|/g, '\\|').replace(/\r?\n/g, ' ')
}
