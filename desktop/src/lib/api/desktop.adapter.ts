import { invoke } from "@tauri-apps/api/core";
import type {
  ProcessSession,
  NewSessionInput,
} from "../../features/sessions/types";

export const desktopAdapter = {
  listSessions(): Promise<ProcessSession[]> {
    return invoke<ProcessSession[]>("list_sessions");
  },

  startSession(input: NewSessionInput): Promise<ProcessSession> {
    return invoke<ProcessSession>("start_session", input);
  },
};
