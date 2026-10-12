import { useMemo, useState } from "react";
import { Icon } from "../../../components/ui/Icon";
import { api } from "../../../lib/api";
import { useSessions } from "../hooks/useSessions";
import type { ProcessSession } from "../types";

type Filter = "all" | "running" | "completed" | "failed";

function getOutcome(session: ProcessSession) {
  if (session.status === "running") return "running";
  if (session.exit_code === 0) return "completed";
  if (session.exit_code != null) return "failed";
  return "exited";
}

function formatStartTime(session: ProcessSession) {
  return new Date(session.start_time_seconds * 1000).toLocaleString();
}

export function ActivityPage() {
  const { sessions, loading, error, refresh } = useSessions();
  const [filter, setFilter] = useState<Filter>("all");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [logs, setLogs] = useState("");
  const [logsLoading, setLogsLoading] = useState(false);
  const [logsError, setLogsError] = useState<string | null>(null);

  const counts = useMemo(() => {
    return {
      running: sessions.filter((s) => getOutcome(s) === "running").length,
      completed: sessions.filter((s) => getOutcome(s) === "completed").length,
      failed: sessions.filter((s) => getOutcome(s) === "failed").length,
    };
  }, [sessions]);

  const filteredSessions = useMemo(() => {
    return [...sessions]
      .filter((session) => {
        const outcome = getOutcome(session);
        if (filter === "all") return true;
        if (filter === "running") return outcome === "running";
        if (filter === "completed") return outcome === "completed";
        return outcome === "failed";
      })
      .sort(
        (a, b) =>
          b.start_time_seconds - a.start_time_seconds ||
          b.start_time_useconds - a.start_time_useconds,
      );
  }, [sessions, filter]);

  async function viewLogs(session: ProcessSession) {
    setSelectedId(session.id);
    setLogs("");
    setLogsError(null);
    setLogsLoading(true);

    try {
      const result = await api.getSessionLogs(session.id);
      setLogs(result);
    } catch (cause) {
      setLogsError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setLogsLoading(false);
    }
  }

  return (
    <section className="page-content activity-page">
      <div className="eyebrow">
        <span className="eyebrow-line" /> DEVBOX WORKSPACE
      </div>

      <div className="activity-heading">
        <div>
          <h1>
            Activity<span className="heading-period">.</span>
          </h1>
          <p className="activity-subtitle">
            Review process outcomes and inspect session logs.
          </p>
        </div>

        <button className="secondary-button" onClick={() => void refresh()}>
          <Icon name="refresh" size={15} />
          Refresh
        </button>
      </div>

      <div className="activity-stats">
        <button
          className={`activity-stat ${filter === "running" ? "is-selected" : ""}`}
          onClick={() => setFilter(filter === "running" ? "all" : "running")}
        >
          <span>Running</span>
          <strong>{counts.running}</strong>
        </button>

        <button
          className={`activity-stat ${filter === "completed" ? "is-selected" : ""}`}
          onClick={() =>
            setFilter(filter === "completed" ? "all" : "completed")
          }
        >
          <span>Completed</span>
          <strong>{counts.completed}</strong>
        </button>

        <button
          className={`activity-stat ${filter === "failed" ? "is-selected" : ""}`}
          onClick={() => setFilter(filter === "failed" ? "all" : "failed")}
        >
          <span>Failed</span>
          <strong>{counts.failed}</strong>
        </button>
      </div>

      <div className="activity-toolbar">
        <div className="activity-filters">
          {(["all", "running", "completed", "failed"] as Filter[]).map(
            (value) => (
              <button
                key={value}
                className={`activity-filter ${filter === value ? "is-active" : ""}`}
                onClick={() => setFilter(value)}
              >
                {value[0].toUpperCase() + value.slice(1)}
              </button>
            ),
          )}
        </div>

        <span className="activity-count">
          {filteredSessions.length} session
          {filteredSessions.length === 1 ? "" : "s"}
        </span>
      </div>

      {error && <p className="activity-error">{error}</p>}

      {loading ? (
        <div className="activity-empty">Loading sessions…</div>
      ) : filteredSessions.length === 0 ? (
        <div className="activity-empty">
          <Icon name="activity" size={22} />
          <strong>No matching activity</strong>
          <span>Try another filter or start a new process.</span>
        </div>
      ) : (
        <div className="activity-list">
          {filteredSessions.map((session) => {
            const outcome = getOutcome(session);

            return (
              <article className="activity-item" key={session.id}>
                <div
                  className={`activity-marker activity-marker--${outcome}`}
                />

                <div className="activity-item-main">
                  <div className="activity-item-title">
                    <strong>{session.name || "Unnamed process"}</strong>
                    <span
                      className={`activity-outcome activity-outcome--${outcome}`}
                    >
                      {outcome === "completed"
                        ? "Completed"
                        : outcome === "failed"
                          ? `Failed · ${session.exit_code}`
                          : outcome === "running"
                            ? "Running"
                            : "Exited · code unavailable"}
                    </span>
                  </div>

                  <code className="activity-command" title={session.command}>
                    {session.command}
                  </code>

                  <div className="activity-meta">
                    <span>PID {session.pid}</span>
                    <span>{formatStartTime(session)}</span>
                  </div>

                  {selectedId === session.id && (
                    <div className="activity-log-panel">
                      <div className="activity-log-heading">
                        <strong>Session logs</strong>
                        <button
                          className="row-action"
                          onClick={() => setSelectedId(null)}
                          aria-label="Close logs"
                          title="Close logs"
                        >
                          ×
                        </button>
                      </div>

                      {logsLoading ? (
                        <p>Loading logs…</p>
                      ) : logsError ? (
                        <p className="activity-error">{logsError}</p>
                      ) : (
                        <pre>{logs || "No logs available."}</pre>
                      )}
                    </div>
                  )}
                </div>

                <button
                  className="secondary-button activity-log-button"
                  onClick={() => void viewLogs(session)}
                >
                  View Logs
                </button>
              </article>
            );
          })}
        </div>
      )}
    </section>
  );
}
