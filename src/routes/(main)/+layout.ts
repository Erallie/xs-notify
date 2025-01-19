import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./settings/$types";
export const load: PageLoad = async () => {
  let isRunning = await invoke<boolean>("get_running");

  return { isRunning };
};
