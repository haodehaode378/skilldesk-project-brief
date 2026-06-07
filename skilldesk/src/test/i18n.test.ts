import { describe, expect, it } from 'vitest'

import { appCopy, nextLocale } from '../app/i18n'

describe('i18n copy', () => {
  it('uses Chinese navigation copy by default', () => {
    expect(appCopy['zh-CN'].nav.overview).toBe('概览')
    expect(appCopy['zh-CN'].dashboard.scanButton).toBe('扫描本地来源')
  })

  it('switches between supported locales', () => {
    expect(nextLocale('zh-CN')).toBe('en-US')
    expect(nextLocale('en-US')).toBe('zh-CN')
  })

  it('keeps Chinese and English copy keys aligned', () => {
    expect(flattenKeys(appCopy['zh-CN'])).toEqual(flattenKeys(appCopy['en-US']))
  })
})

function flattenKeys(value: unknown, prefix = ''): string[] {
  if (!value || typeof value !== 'object') {
    return [prefix]
  }

  return Object.entries(value)
    .flatMap(([key, child]) => flattenKeys(child, prefix ? `${prefix}.${key}` : key))
    .sort()
}
