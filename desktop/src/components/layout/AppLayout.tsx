import type { ReactNode } from "react";
import { Sidebar } from "./Sidebar";
import { Topbar } from "./Topbar";
import type { NavItem } from "./navigation";

type Props = {
  activePage: NavItem;
  onNavigate: (page: NavItem) => void;
  children: ReactNode;
};

export function AppLayout({ activePage, onNavigate, children }: Props) {
  return (
    <div className="app-shell">
      <Sidebar activePage={activePage} onNavigate={onNavigate} />
      <main className="main-area">
        <Topbar page={activePage} />
        {children}
      </main>
    </div>
  );
}
