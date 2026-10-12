import type { ProcessSession } from "../../features/sessions/types";

type Props = {
  session: ProcessSession;
};

export function ProcessStatusBadge({ session }: Props) {
  const isRunning = session.status === "running";

  return (
    <span
      className={`process-status ${
        isRunning ? "process-status--running" : "process-status--exited"
      }`}
    >
      <span className="process-status__dot" />

      {isRunning
        ? "Running"
        : session.exit_code == null
          ? "Exited"
          : `Exited · ${session.exit_code}`}
    </span>
  );
}
