import { Icon } from "../../../components/ui/Icon";
import { formatStarted } from "../utils/formatStarted";
import type { ProcessSession } from "../types";
import { useState } from "react";

type Props = {
  sessions: ProcessSession[];
  onStop: (id: string) => Promise<void>;
};

export function SessionTable({ sessions, onStop }: Props) {
  if (sessions.length === 0) return null;
  const [stoppingId, setStoppingId] = useState<string | null>(null);
  const [stopError, setStopError] = useState<string | null>(null);

  async function handleStop(id: string) {
    setStoppingId(id);
    setStopError(null);

    try {
      await onStop(id);
    } catch (cause) {
      setStopError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setStoppingId(null);
    }
  }

  return (
    <div className="session-table-wrap">
      {stopError && (
        <p role="alert" className="error-message">
          {stopError}
        </p>
      )}

      <table className="session-table">
        <thead>
          <tr>
            <th>PROCESS</th>
            <th>COMMAND</th>
            <th>PID</th>
            <th>STARTED</th>
            <th>
              <span className="sr-only">Actions</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {sessions.map((session) => (
            <tr key={session.id}>
              <td>
                <div className="process-cell">
                  <div className="process-icon">
                    <Icon name="terminal" size={17} />
                  </div>
                  <div className="process-copy">
                    <strong>{session.name || "Unnamed process"}</strong>
                    <span title={session.id}>{session.id.slice(0, 8)}…</span>
                  </div>
                </div>
              </td>
              <td>
                <code className="command-cell" title={session.command}>
                  {session.command}
                </code>
              </td>
              <td>
                <span className="pid-cell">{session.pid}</span>
              </td>
              <td>
                <span className="started-cell">
                  <Icon name="clock" size={14} />
                  {formatStarted(session)}
                </span>
              </td>
              <td>
                <div className="row-actions">
                  <button
                    className="row-action"
                    aria-label={`Copy session ID for ${session.name}`}
                    onClick={() =>
                      void navigator.clipboard?.writeText(session.id)
                    }
                    title="Copy session ID"
                  >
                    <Icon name="copy" size={16} />
                  </button>

                  <button
                    className="row-action"
                    aria-label={`Stop session ${session.name}`}
                    onClick={() => void handleStop(session.id)}
                    disabled={stoppingId === session.id}
                    title="Stop session"
                  >
                    {stoppingId === session.id ? (
                      <Icon name="loader" size={16} />
                    ) : (
                      <Icon name="square" size={16} />
                    )}
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="table-footer">
        <span>
          Showing <strong>{sessions.length}</strong> session
          {sessions.length === 1 ? "" : "s"}
        </span>
        <span className="live-indicator">
          <span /> Synced on refresh
        </span>
      </div>
    </div>
  );
}
