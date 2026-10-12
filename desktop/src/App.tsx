import { useState } from "react";
import { AppLayout } from "./components/layout/AppLayout";
import { PlaceholderPage } from "./components/layout/PlaceholderPage";
import type { NavItem } from "./components/layout/navigation";
import { ActivityPage } from "./features/sessions/pages/ActivityPage";
import { SessionsPage } from "./features/sessions/pages/SessionsPage";
import "./styles/global.css";

export default function App() {
  const [activePage, setActivePage] = useState<NavItem>("sessions");

  return (
    <AppLayout activePage={activePage} onNavigate={setActivePage}>
      {activePage === "sessions" ? (
        <SessionsPage />
      ) : activePage === "activity" ? (
        <ActivityPage />
      ) : (
        <PlaceholderPage page={activePage} />
      )}
    </AppLayout>
  );
}
