import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import type { Logs } from "$lib/types/types.js";


export const load: PageLoad = async () => {
    let logs = await invoke<Logs[]>("load_logs");

    console.log(logs);

    return { logs };
};