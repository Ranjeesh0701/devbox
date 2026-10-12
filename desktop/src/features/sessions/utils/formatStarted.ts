import type { ProcessSession } from "../types";

export function formatStarted(session: ProcessSession): string {
  const date = new Date(
    session.start_time_seconds * 1000 +
    Math.floor(session.start_time_useconds / 1000),
  );
  if (Number.isNaN(date.getTime())) return "Unknown";
  return new Intl.DateTimeFormat(undefined, {
    hour: "2-digit", minute: "2-digit", month: "short", day: "numeric",
  }).format(date);
}
