import { renderToString } from 'react-dom/server'
import { describe, expect, it } from 'vitest'

import App from '../App'
import { appCopy } from '../app/i18n'
import { fixtureScanReport } from '../fixtures'

describe('App fixture rendering', () => {
  it('renders a non-empty Chinese overview from fixture data', () => {
    const html = renderToString(<App />)

    expect(html).toContain('SkillDesk')
    expect(html).toContain(appCopy['zh-CN'].dashboard.title)
    expect(html).toContain(appCopy['zh-CN'].dashboard.scanButton)
    expect(html).toContain(String(fixtureScanReport.totals.entities))
  })

  it('renders the extension list and selected detail from fixture data', () => {
    const html = renderToString(<App initialView="extensions" />)
    const firstEntity = fixtureScanReport.entities[0]

    expect(html).toContain(appCopy['zh-CN'].views.extensionsTitle)
    expect(html).toContain(firstEntity.title ?? firstEntity.name)
    expect(html).toContain(appCopy['zh-CN'].views.detailTitle)
  })
})
