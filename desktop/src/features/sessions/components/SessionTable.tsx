import { Icon } from "../../../components/ui/Icon";
import { formatStarted } from "../utils/formatStarted";
import type { ProcessSession } from "../types";

type Props = { sessions: ProcessSession[] };

export function SessionTable({ sessions }: Props) {
  if (sessions.length === 0) return null;
  return (
    <div className="session-table-wrap">
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
                <button
                  className="row-action"
                  aria-label={`Copy session ID for ${session.name}`}
                  onClick={() =>
                    void navigator.clipboard?.writeText(session.id)
                  }
                  title="Copy session ID"
                >
                  <Icon name="more" size={18} />
                </button>
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
