import "./App.css";

const processes = [
  {
    name: "frontend",
    command: "bun run dev",
    status: "running",
    detail: "Web application",
  },
  {
    name: "backend",
    command: "cargo run",
    status: "running",
    detail: "API server",
  },
  {
    name: "tests",
    command: "bun run test",
    status: "stopped",
    detail: "Last run · exited",
  },
] as const;

const features = [
  {
    number: "01",
    icon: "⌘",
    title: "Built for AI workflows",
    description:
      "Give coding agents a consistent way to start and manage development processes instead of juggling terminal sessions.",
    tag: "AGENT-FRIENDLY",
  },
  {
    number: "02",
    icon: "⌁",
    title: "See what is actually running",
    description:
      "View process status, inspect output, and understand when a server or task stops without searching through terminal tabs.",
    tag: "VISUAL CONTROL",
  },
  {
    number: "03",
    icon: "▤",
    title: "One shared view",
    description:
      "Keep your frontend, backend, workers, and other development processes visible in one workspace for you and your tools.",
    tag: "ONE WORKSPACE",
  },
];

function ArrowIcon() {
  return <span aria-hidden="true">↗</span>;
}

function ProcessPreview() {
  return (
    <div className="app-window terminal-window">
      <div className="window-titlebar">
        <div className="window-dots" aria-hidden="true">
          <span />
          <span />
          <span />
        </div>
        <span className="window-title">devbox — local workspace</span>
        <span className="window-version">PREVIEW</span>
      </div>

      <div className="terminal-body">
        <div className="terminal-context">
          <span className="agent-indicator">
            <span className="agent-indicator-dot" />
            AI-ASSISTED WORKFLOW
          </span>
          <span className="terminal-muted">Local processes</span>
        </div>

        <div className="terminal-command">
          <span className="prompt-symbol">❯</span>
          <span>devbox list</span>
        </div>

        <div className="terminal-intro">
          <span className="terminal-accent">devbox</span>
          <span className="terminal-muted"> / sessions</span>
        </div>

        <div className="terminal-table">
          <div className="terminal-table-header">
            <span>PROCESS</span>
            <span>COMMAND</span>
            <span>STATUS</span>
          </div>

          {processes.map((process) => (
            <div className="terminal-process-row" key={process.name}>
              <span className="terminal-process-name">{process.name}</span>
              <code>{process.command}</code>
              <span className={`terminal-status ${process.status}`}>
                <span className="status-dot" />
                {process.status === "running" ? "Running" : "Stopped"}
              </span>
            </div>
          ))}
        </div>

        <div className="terminal-bottom">
          <div className="terminal-summary">
            <span className="terminal-green">● 2 running</span>
            <span className="terminal-muted">1 stopped</span>
          </div>
          <span className="preview-label">ILLUSTRATIVE PREVIEW</span>
        </div>
      </div>
    </div>
  );
}

function WorkflowDiagram() {
  return (
    <div className="workflow-diagram">
      <div className="workflow-node">
        <div className="workflow-node-icon agent-node-icon">
          <span>✳</span>
        </div>
        <div className="workflow-node-copy">
          <strong>AI coding agent</strong>
          <span>Writes code and requests processes</span>
        </div>
        <span className="workflow-node-number">01</span>
      </div>

      <div className="workflow-connector">
        <span />
        <span className="workflow-connector-label">Process commands</span>
      </div>

      <div className="workflow-node workflow-node-highlight">
        <div className="workflow-node-icon devbox-node-icon">
          <span>d</span>
        </div>
        <div className="workflow-node-copy">
          <strong>Devbox</strong>
          <span>Runs and tracks development processes</span>
        </div>
        <span className="workflow-node-number">02</span>
      </div>

      <div className="workflow-connector">
        <span />
        <span className="workflow-connector-label">
          Process state and output
        </span>
      </div>

      <div className="workflow-node">
        <div className="workflow-node-icon dashboard-node-icon">
          <span>▦</span>
        </div>
        <div className="workflow-node-copy">
          <strong>Developer dashboard</strong>
          <span>See status, inspect logs, stay in control</span>
        </div>
        <span className="workflow-node-number">03</span>
      </div>
    </div>
  );
}

function App() {
  return (
    <div className="site-shell">
      <header className="site-header">
        <a className="brand" href="#home" aria-label="Devbox home">
          <span className="brand-mark">d</span>
          <span>devbox</span>
        </a>

        <nav className="main-nav" aria-label="Main navigation">
          <a href="#features">Why Devbox</a>
          <a href="#workflow">How it works</a>
          <a href="https://github.com/" target="_blank" rel="noreferrer">
            GitHub <ArrowIcon />
          </a>
        </nav>

        <a className="header-cta" href="#get-started">
          Explore Devbox <ArrowIcon />
        </a>
      </header>

      <main>
        <section className="hero section-container" id="home">
          <div className="hero-copy">
            <div className="eyebrow">
              <span className="eyebrow-dot" />
              DEVELOPMENT IN THE AGE OF AI
            </div>

            <h1>
              Let AI build.
              <br />
              <span>Stay in control.</span>
            </h1>

            <p className="hero-description">
              AI can write your code and start your servers. Devbox is being
              built to help you see what's running, inspect process output, and
              manage your development workflow from one place.
            </p>

            <div className="hero-actions">
              <a className="button button-primary" href="#workflow">
                See how it works <ArrowIcon />
              </a>
              <a className="button button-secondary" href="#features">
                Why Devbox <span aria-hidden="true">↓</span>
              </a>
            </div>

            <div className="hero-note">
              <span className="note-check">✓</span>
              Built around your workflow, not another AI chatbot.
            </div>
          </div>

          <div className="hero-visual">
            <div className="visual-glow" />
            <div className="visual-label">
              <span className="visual-label-dot" />
              YOUR DEVELOPMENT WORKSPACE
            </div>
            <ProcessPreview />
            <div className="floating-chip">
              <span className="status-dot" />
              Your processes, at a glance
            </div>
          </div>
        </section>

        <section className="principles">
          <div className="section-container principles-inner">
            <p>FROM AI-GENERATED CODE TO RUNNING SOFTWARE</p>
            <div className="principle-items">
              <span>
                <i>01</i> Build
              </span>
              <span>
                <i>02</i> Run
              </span>
              <span>
                <i>03</i> Observe
              </span>
            </div>
          </div>
        </section>

        <section className="features section-container" id="features">
          <div className="section-heading">
            <div className="eyebrow">THE MISSING VISIBILITY LAYER</div>
            <h2>Let your agent work. See what it runs.</h2>
            <p>
              AI-assisted development shouldn't mean losing track of the
              processes that power your application.
            </p>
          </div>

          <div className="feature-grid">
            {features.map((feature) => (
              <article className="feature-card" key={feature.number}>
                <div className="feature-card-top">
                  <span className="feature-icon">{feature.icon}</span>
                  <span className="feature-number">{feature.number}</span>
                </div>
                <span className="feature-tag">{feature.tag}</span>
                <h3>{feature.title}</h3>
                <p>{feature.description}</p>
                <a href="#workflow" className="feature-link">
                  How it fits <ArrowIcon />
                </a>
              </article>
            ))}
          </div>
        </section>

        <section className="workflow-section" id="workflow">
          <div className="section-container workflow-inner">
            <div className="workflow-copy">
              <div className="eyebrow">A BETTER DEVELOPMENT LOOP</div>
              <h2>From agent instructions to application insight.</h2>
              <p>
                Imagine a workflow where your coding agent can manage processes
                through a predictable interface, while you get a clear visual
                view of what's running and what needs your attention.
              </p>

              <div className="workflow-benefits">
                <div>
                  <span className="benefit-check">✓</span>
                  <span>Consistent process management</span>
                </div>
                <div>
                  <span className="benefit-check">✓</span>
                  <span>Visible process state and output</span>
                </div>
                <div>
                  <span className="benefit-check">✓</span>
                  <span>A human-readable workspace</span>
                </div>
              </div>
            </div>

            <WorkflowDiagram />
          </div>
        </section>

        <section className="get-started section-container" id="get-started">
          <div className="get-started-panel">
            <div className="eyebrow">
              THE FUTURE OF DEVELOPMENT IS DIFFERENT
            </div>
            <h2>Build with AI. Understand what runs.</h2>
            <p>
              Devbox is evolving to make local development processes easier to
              manage in an AI-assisted world.
            </p>
            <a
              className="button button-primary"
              href="https://github.com/"
              target="_blank"
              rel="noreferrer"
            >
              Explore the project <ArrowIcon />
            </a>
            <span className="get-started-note">
              Replace this link with the actual Devbox repository URL.
            </span>
          </div>
        </section>
      </main>

      <footer className="site-footer">
        <a className="brand footer-brand" href="#home">
          <span className="brand-mark">d</span>
          <span>devbox</span>
        </a>
        <p>A clearer view of the processes behind your code.</p>
        <span className="footer-copyright">
          © {new Date().getFullYear()} Devbox
        </span>
      </footer>
    </div>
  );
}

export default App;
