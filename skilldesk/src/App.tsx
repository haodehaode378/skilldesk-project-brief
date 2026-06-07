import { useState } from 'react'

import { appCopy, nextLocale } from './app/i18n'
import './App.css'
import { fixtureScanReport } from './fixtures'
import type { Locale } from './model'

function App() {
  const [locale, setLocale] = useState<Locale>('zh-CN')
  const copy = appCopy[locale]
  const report = fixtureScanReport
  const totals = report.totals

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
          <a aria-current="page" href="#overview">
            {copy.nav.overview}
          </a>
          <a href="#extensions">{copy.nav.extensions}</a>
          <a href="#mcp">{copy.nav.mcpServers}</a>
          <a href="#plugins">{copy.nav.plugins}</a>
          <a href="#sources">{copy.nav.sources}</a>
          <a href="#issues">{copy.nav.issues}</a>
          <a href="#settings">{copy.nav.settings}</a>
        </nav>
      </aside>

      <section className="workspace" id="overview">
        <header className="topbar">
          <div>
            <p className="eyebrow">{copy.dashboard.eyebrow}</p>
            <h2>{copy.dashboard.title}</h2>
          </div>
          <div className="topbar-actions">
            <button type="button" className="secondary-button" onClick={() => setLocale(nextLocale(locale))}>
              {copy.dashboard.languageButton}
            </button>
            <button type="button">{copy.dashboard.scanButton}</button>
          </div>
        </header>

        <section className="summary-grid" aria-label="Health summary">
          <article>
            <span>{copy.dashboard.summary.totalExtensions}</span>
            <strong>{totals.entities}</strong>
          </article>
          <article>
            <span>{copy.dashboard.summary.needsReview}</span>
            <strong>{totals.byStatus['needs-review']}</strong>
          </article>
          <article>
            <span>{copy.dashboard.summary.mcpServers}</span>
            <strong>{totals.mcpServers}</strong>
          </article>
          <article>
            <span>{copy.dashboard.summary.broken}</span>
            <strong>{totals.byStatus.broken}</strong>
          </article>
          <article>
            <span>{copy.dashboard.summary.skills}</span>
            <strong>{totals.skills}</strong>
          </article>
          <article>
            <span>{copy.dashboard.summary.plugins}</span>
            <strong>{totals.plugins}</strong>
          </article>
        </section>

        <section className="panel">
          <div>
            <h3>{copy.dashboard.panelTitle}</h3>
            <p>{copy.dashboard.panelBody}</p>
          </div>
          <code>{copy.dashboard.phaseTag}</code>
        </section>
      </section>
    </main>
  )
}

export default App
