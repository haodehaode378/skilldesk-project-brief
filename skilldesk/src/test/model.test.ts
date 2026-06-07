import { describe, expect, it } from 'vitest'

import { fixtureEntities, fixtureScanReport } from '../fixtures'
import {
  appSettingsSchema,
  calculateScanTotals,
  defaultAppSettings,
  scanReportSchema,
} from '../model'

describe('scan report model', () => {
  it('validates the fixture scan report', () => {
    expect(() => scanReportSchema.parse(fixtureScanReport)).not.toThrow()
  })

  it('calculates entity and status totals', () => {
    expect(calculateScanTotals(fixtureEntities)).toEqual({
      entities: 4,
      skills: 1,
      commands: 1,
      agents: 0,
      plugins: 1,
      mcpServers: 1,
      instructionFiles: 0,
      byStatus: {
        ok: 2,
        'needs-review': 2,
        'at-risk': 0,
        broken: 0,
      },
    })
  })

  it('defaults to Chinese UI locale', () => {
    expect(appSettingsSchema.parse(defaultAppSettings).locale).toBe('zh-CN')
  })

  it('includes read-only MVP default scan roots', () => {
    const settings = appSettingsSchema.parse(defaultAppSettings)

    expect(settings.scanRoots).toContain('%USERPROFILE%\\.codex\\skills')
    expect(settings.scanRoots).toContain('%USERPROFILE%\\.claude\\commands')
    expect(settings.scanRoots).toContain('%USERPROFILE%\\.codex\\plugins')
    expect(settings.mcpProbePolicy).toBe('disabled')
    expect(settings.includePluginCaches).toBe(false)
  })
})
