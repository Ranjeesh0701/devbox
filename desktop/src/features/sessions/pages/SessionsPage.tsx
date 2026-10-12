import { useMemo, useState } from "react";
import { Button } from "../../../components/ui/Button";
import { Icon } from "../../../components/ui/Icon";
import { SessionEmptyState } from "../components/SessionEmptyState";
import { SessionStats } from "../components/SessionStats";
import { SessionTable } from "../components/SessionTable";
import { useSessions } from "../hooks/useSessions";

export function SessionsPage() {
  const { sessions, error, loading, lastUpdated, refresh } = useSessions();
  const [search, setSearch] = useState("");

  const filteredSessions = useMemo(() => {
    const query = search.trim().toLowerCase();
    if (!query) return sessions;
    return sessions.filter((session) =>
      [session.name, session.command, session.id, String(session.pid)]
        .some((value) => value.toLowerCase().includes(query)),
    );
  }, [sessions, search]);

  return (
    <div className="page-content">
      <div className="page-heading">
        <div><div className="eyebrow"><span className="eyebrow-line" /> LOCAL ENVIRONMENT</div><h1>Sessions<span className="heading-period">.</span></h1><p className="page-description">Your processes, all in one place.</p></div>
        <Button variant="primary" icon={<Icon name="plus" size={17} />} onClick={() => window.alert("Start-session support will be connected in the next milestone.")}>New session <span className="button-shortcut">⌘ N</span></Button>
      </div>

      <SessionStats count={sessions.length} loading={loading} connected={!error} lastUpdated={lastUpdated} />

      <section className="sessions-section">
        <div className="section-heading"><div><h2>All sessions <span className="section-count">{sessions.length}</span></h2><p>Processes managed by your Devbox daemon</p></div><Button icon={<span className={loading ? "refresh-spinning" : ""}><Icon name="refresh" size={15} /></span>} onClick={() => void refresh()} disabled={loading}>{loading ? "Refreshing" : "Refresh"}</Button></div>
        <div className="table-toolbar">
          <label className="search-field"><Icon name="search" size={17} /><input value={search} onChange={(event) => setSearch(event.target.value)} placeholder="Search sessions..." aria-label="Search sessions" /><kbd>⌘ F</kbd></label>
          <div className="view-label"><span className="view-dot" /> All processes</div>
        </div>

        {error ? <SessionEmptyState kind="error" message={error} onRetry={() => void refresh()} />
          : loading && sessions.length === 0 ? <SessionEmptyState kind="loading" />
          : filteredSessions.length === 0 ? <SessionEmptyState kind={search ? "no-results" : "empty"} />
          : <SessionTable sessions={filteredSessions} />}
      </section>

      <footer className="page-footer"><span>Devbox Desktop <span className="footer-version">v0.1.0</span></span><span><span className="footer-status-dot" /> Local-first process management</span></footer>
    </div>
  );
}
