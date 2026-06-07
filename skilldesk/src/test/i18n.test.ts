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
})
