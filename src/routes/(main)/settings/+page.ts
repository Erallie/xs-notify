import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import {
    type XSNotifySettings,
} from "$lib/types/types";


export const load: PageLoad = async () => {
    let settings = await invoke<XSNotifySettings>("get_settings");

    // Below are the default values
    /* let settings: XSNotifySettings = {
        autoRun: true,
        port: 42069,
        host: "localhost",
        pollingRate: 250,
        dynamicTimeout: true,
        defaultTimeout: 5.0,
        readingSpeed: 238.0,
        minTimeout: 2.0,
        maxTimeout: 120.0,
        // app_list: Vec::new(),
        appList: ["App 1", "App 2"],
    }; */

    return { settings };
};