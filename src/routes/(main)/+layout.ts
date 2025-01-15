// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./settings/$types";
export const prerender = true;
export const ssr = false;

export const load: PageLoad = async () => {

    let isRunning = await invoke<boolean>("get_running");

    return { isRunning };
};