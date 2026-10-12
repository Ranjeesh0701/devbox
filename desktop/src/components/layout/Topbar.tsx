import { Icon } from "../ui/Icon";
import type { NavItem } from "./navigation";

const labels: Record<NavItem, string> = {
  sessions: "Sessions",
  activity: "Activity",
  settings: "Settings",
};

export function Topbar({ page }: { page: NavItem }) {
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
      </div>
    </header>
  );
}
