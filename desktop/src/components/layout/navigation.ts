export type NavItem = "sessions" | "activity" | "settings";

export const NAV_ITEMS: {
  id: NavItem;
  label: string;
  icon: "grid" | "activity" | "settings";
}[] = [
  { id: "sessions", label: "Sessions", icon: "grid" },
  { id: "activity", label: "Activity", icon: "activity" },
  { id: "settings", label: "Settings", icon: "settings" },
];
