// Components/features depend on this API boundary rather than Tauri directly.
// A web adapter can implement the same contract later.
export { desktopAdapter as api } from "./desktop.adapter";
