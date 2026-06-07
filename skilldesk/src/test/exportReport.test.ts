import { describe, expect, it } from 'vitest'

import { buildExportPayload } from '../app/exportReport'
import { fixtureScanReport } from '../fixtures'
import { defaultAppSettings } from '../model'

describe('buildExportPayload', () => {
  it('adds report summary, scanner self-check, and capability metadata', () => {
    const payload = buildExportPayload({
      exportedAt: '2026-06-07T12:00:00.000Z',
      report: fixtureScanReport,
      reportSource: 'local',
      settings: defaultAppSettings,
    })

    expect(payload.exportSchemaVersion).toBe('0.1')
    expect(payload.reportSource).toBe('local')
    expect(payload.reportSummary).toEqual({
      generatedAt: fixtureScanReport.generatedAt,
      schemaVersion: '0.1',
      machinePlatform: 'win32',
      roots: 2,
      entities: 4,
      issues: 2,
    })
    expect(payload.scannerSelfCheck).toEqual({
      scanned: 1,
      missing: 1,
      skipped: 0,
      errors: 0,
    })
    expect(payload.capabilities).toEqual({
      readOnlyScanning: true,
      pluginCacheMode: 'summary-only',
      mcpProbePolicy: 'disabled',
    })
    expect(payload.scanReport).toBe(fixtureScanReport)
  })
})
