import { describe, expect, it } from 'vitest'

import {
  loadCachedReport,
  loadLocale,
  saveCachedReport,
  saveLocale,
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

  it('saves and loads the latest scan report', () => {
    const storage = createStorage()

    saveCachedReport(fixtureScanReport, storage)

    expect(loadCachedReport(storage)?.totals.entities).toBe(
      fixtureScanReport.totals.entities,
    )
  })

  it('clears invalid cached scan reports', () => {
    const storage = createStorage()
    storage.setItem('skilldesk.lastReport', '{bad json')

    expect(loadCachedReport(storage)).toBeUndefined()
    expect(storage.getItem('skilldesk.lastReport')).toBeNull()
  })
})
