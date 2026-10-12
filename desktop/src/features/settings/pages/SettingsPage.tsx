import type { Theme } from "../../../lib/theme";

type Props = {
  theme: Theme;
  onThemeChange: (theme: Theme) => void;
};

export function SettingsPage({ theme, onThemeChange }: Props) {
  return (
    <section className="page-content settings-page">
      <div className="eyebrow">
        <span className="eyebrow-line" /> DEVBOX WORKSPACE
      </div>

      <div className="settings-heading">
        <div>
          <h1>
            Settings<span className="heading-period">.</span>
          </h1>
          <p className="page-description">Customize your Devbox workspace.</p>
        </div>
      </div>

      <section className="settings-section">
        <div className="settings-section-heading">
          <div className="settings-section-icon">◐</div>
          <div>
            <h2>Appearance</h2>
            <p>Choose how Devbox looks on your device.</p>
          </div>
        </div>

        <div className="settings-row">
          <div className="settings-row-copy">
            <strong>Color theme</strong>
            <span>Your preference applies throughout the app.</span>
          </div>

          <div className="theme-options" role="group" aria-label="Color theme">
            <button
              className={`theme-option ${theme === "light" ? "is-selected" : ""}`}
              aria-pressed={theme === "light"}
              onClick={() => onThemeChange("light")}
            >
              Light
            </button>
            <button
              className={`theme-option ${theme === "dark" ? "is-selected" : ""}`}
              aria-pressed={theme === "dark"}
              onClick={() => onThemeChange("dark")}
            >
              Dark
            </button>
          </div>
        </div>
      </section>

      <section className="settings-section">
        <div className="settings-section-heading">
          <div className="settings-section-icon">⌘</div>
          <div>
            <h2>Process defaults</h2>
            <p>Options for launching new processes.</p>
          </div>
        </div>

        <div className="settings-row">
          <div className="settings-row-copy">
            <strong>Default working directory</strong>
            <span>Working-directory preferences aren't configurable yet.</span>
          </div>
          <span className="settings-tag">Coming later</span>
        </div>

        <div className="settings-row">
          <div className="settings-row-copy">
            <strong>Default shell</strong>
            <span>
              Processes currently use the command provided when launched.
            </span>
          </div>
          <span className="settings-tag">Coming later</span>
        </div>
      </section>

      <section className="settings-section">
        <div className="settings-section-heading">
          <div className="settings-section-icon">i</div>
          <div>
            <h2>About</h2>
            <p>Information about your application.</p>
          </div>
        </div>

        <div className="settings-row">
          <div className="settings-row-copy">
            <strong>Devbox Desktop</strong>
            <span>Local process management for developers.</span>
          </div>
          <span className="settings-tag">Developer preview</span>
        </div>

        <div className="settings-row">
          <div className="settings-row-copy">
            <strong>Platform</strong>
            <span>macOS desktop</span>
          </div>
        </div>
      </section>
    </section>
  );
}
