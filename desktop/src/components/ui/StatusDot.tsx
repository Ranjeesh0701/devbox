type Props = { status: "online" | "offline" | "pending"; label?: string };

export function StatusDot({ status, label }: Props) {
  return (
    <span
      className={`status-pip ${status}`}
      aria-label={label ?? status}
      title={label ?? status}
    />
  );
}
