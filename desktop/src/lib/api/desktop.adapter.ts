import { invoke } from "@tauri-apps/api/core";
import type { ProcessSession } from "../../features/sessions/types";

export const desktopAdapter = {
  listSessions(): Promise<ProcessSession[]> {
    return invoke<ProcessSession[]>("list_sessions");
  },
};
