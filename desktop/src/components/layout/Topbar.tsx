import { Icon } from "../ui/Icon";
import type { NavItem } from "./navigation";

const labels: Record<NavItem, string> = {
  sessions: "Sessions",
  activity: "Activity",
  settings: "Settings",
};

export function Topbar({ page }: { page: NavItem }) {
  const theme = document.documentElement.dataset.theme ?? "dark";
  return (
    <header className="topbar">
      <div className="breadcrumbs">
        <span>Workspace</span>
        <Icon name="chevron" size={14} />
        <strong>{labels[page]}</strong>
      </div>
      <div className="topbar-actions">
        <div className="keyboard-hint">
          <Icon name="command" size={13} />
          <span>K</span>
        </div>
        <span className="topbar-divider" />
        <span className="topbar-theme-icon" aria-hidden="true">
          <Icon name={theme === "dark" ? "moon" : "sun"} />
        </span>
        <div className="user-avatar">D</div>
      </div>
    </header>
  );
}
