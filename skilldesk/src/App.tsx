import './App.css'

function App() {
  return (
    <main className="app-shell">
      <aside className="sidebar" aria-label="Primary navigation">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true">
            SD
          </div>
          <div>
            <h1>SkillDesk</h1>
            <p>Local extension health</p>
          </div>
        </div>
        <nav>
          <a aria-current="page" href="#overview">
            Overview
          </a>
          <a href="#extensions">Extensions</a>
          <a href="#mcp">MCP Servers</a>
          <a href="#plugins">Plugins</a>
          <a href="#sources">Sources</a>
          <a href="#issues">Issues</a>
        </nav>
      </aside>

      <section className="workspace" id="overview">
        <header className="topbar">
          <div>
            <p className="eyebrow">Read-only MVP scaffold</p>
            <h2>Agent extension health dashboard</h2>
          </div>
          <button type="button">Scan local roots</button>
        </header>

        <section className="summary-grid" aria-label="Health summary">
          <article>
            <span>Total extensions</span>
            <strong>0</strong>
          </article>
          <article>
            <span>Needs review</span>
            <strong>0</strong>
          </article>
          <article>
            <span>MCP servers</span>
            <strong>0</strong>
          </article>
          <article>
            <span>Broken</span>
            <strong>0</strong>
          </article>
        </section>

        <section className="panel">
          <div>
            <h3>No scan has run yet</h3>
            <p>
              The first implementation pass will connect this shell to fixture
              data, then to the read-only scanner core.
            </p>
          </div>
          <code>Phase 1: scaffold</code>
        </section>
      </section>
    </main>
  )
}

export default App
