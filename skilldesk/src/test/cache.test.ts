import { describe, expect, it } from 'vitest'

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
} from '../app/cache'
import { fixtureScanReport } from '../fixtures'

function createStorage() {
  const values = new Map<string, string>()

  return {
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => {
      values.set(key, value)
    },
    removeItem: (key: string) => {
      values.delete(key)
    },
  }
}

describe('app cache', () => {
  it('loads the Chinese locale by default', () => {
    expect(loadLocale(createStorage())).toBe('zh-CN')
  })

  it('saves and loads the selected locale', () => {
    const storage = createStorage()

    saveLocale('en-US', storage)

    expect(loadLocale(storage)).toBe('en-US')
  })

  it('saves and loads app settings', () => {
    const storage = createStorage()

    saveAppSettings(
      {
        locale: 'en-US',
        scanRoots: ['%USERPROFILE%\\.codex\\skills'],
        includePluginCaches: true,
        mcpProbePolicy: 'disabled',
      },
      storage,
    )

    expect(loadAppSettings(storage)).toEqual({
      locale: 'en-US',
      scanRoots: ['%USERPROFILE%\\.codex\\skills'],
      includePluginCaches: true,
      mcpProbePolicy: 'disabled',
    })
  })

  it('clears invalid app settings', () => {
    const storage = createStorage()
    storage.setItem('skilldesk.settings', '{bad json')

    expect(loadAppSettings(storage).locale).toBe('zh-CN')
    expect(storage.getItem('skilldesk.settings')).toBeNull()
  })

  it('saves and loads view preferences', () => {
    const storage = createStorage()

    saveViewPreferences(
      {
        extensions: {
          statusFilter: 'needs-review',
          kindFilter: 'skill',
          query: 'docs',
          sortKey: 'issues',
          sortDirection: 'desc',
        },
        issues: {
          severityFilter: 'medium',
          query: 'metadata',
        },
      },
      storage,
    )

    expect(loadViewPreferences(storage)).toEqual({
      extensions: {
        statusFilter: 'needs-review',
        kindFilter: 'skill',
        query: 'docs',
        sortKey: 'issues',
        sortDirection: 'desc',
      },
      issues: {
        severityFilter: 'medium',
        query: 'metadata',
      },
    })
  })

  it('saves and loads the latest scan report', () => {
    const storage = createStorage()

    saveCachedReport(fixtureScanReport, storage)

    expect(loadCachedReport(storage)?.totals.entities).toBe(
      fixtureScanReport.totals.entities,
    )
  })

  it('clears cached scan reports', () => {
    const storage = createStorage()

    saveCachedReport(fixtureScanReport, storage)
    clearCachedReport(storage)

    expect(loadCachedReport(storage)).toBeUndefined()
  })

  it('clears invalid cached scan reports', () => {
    const storage = createStorage()
    storage.setItem('skilldesk.lastReport', '{bad json')

    expect(loadCachedReport(storage)).toBeUndefined()
    expect(storage.getItem('skilldesk.lastReport')).toBeNull()
  })
})
