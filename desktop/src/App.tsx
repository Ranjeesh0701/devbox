import { useEffect, useState } from "react";
import { AppLayout } from "./components/layout/AppLayout";
import { PlaceholderPage } from "./components/layout/PlaceholderPage";
import type { NavItem } from "./components/layout/navigation";
import { ActivityPage } from "./features/sessions/pages/ActivityPage";
import { SessionsPage } from "./features/sessions/pages/SessionsPage";
import { SettingsPage } from "./features/settings/pages/SettingsPage";
import { applyTheme, getInitialTheme, type Theme } from "./lib/theme";
import "./styles/global.css";

export default function App() {
  const [activePage, setActivePage] = useState<NavItem>("sessions");
  const [theme, setTheme] = useState<Theme>(getInitialTheme);

  useEffect(() => {
    applyTheme(theme);
  }, [theme]);

  return (
    <AppLayout
      activePage={activePage}
      onNavigate={setActivePage}
      theme={theme}
      onThemeChange={setTheme}
    >
      {activePage === "sessions" ? (
        <SessionsPage />
      ) : activePage === "activity" ? (
        <ActivityPage />
      ) : activePage === "settings" ? (
        <SettingsPage theme={theme} onThemeChange={setTheme} />
      ) : (
        <PlaceholderPage page={activePage} />
      )}
    </AppLayout>
  );
}
