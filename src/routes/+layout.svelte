<script lang="ts">
    import "../app.pcss";
    import { setContext } from "svelte";
    import type { Logs } from "$lib/types/types.js";
    import { attachLogger } from "@tauri-apps/plugin-log";

    let { children } = $props();

    // with TargetKind::Webview enabled this function will print logs to the browser console

    let detachLogger: () => void;
    // Use $effect to handle side effects
    $effect(() => {
        // Create an async function to handle the logger attachment
        const attachLoggerAsync = async () => {
            detachLogger = await attachLogger((value) => {
                addToConsole(value.message, value.level);
            });
        };

        // Call the async function
        attachLoggerAsync();

        // Cleanup function to detach the logger
        return () => {
            if (detachLogger) {
                detachLogger();
            }
        };
    });

    let logElement: Logs[] = $state([]);
    setContext("logElement", logElement);

    function addToConsole(message: string, level: number) {
        let consoleEl = document.getElementById("console-el");

        let isAtBottom = false;
        if (
            consoleEl &&
            consoleEl.scrollTop >=
                consoleEl.scrollHeight - consoleEl.clientHeight * 1.5
        ) {
            isAtBottom = true;
        }
        let cls = "text-gray-500";
        let extraClasses;
        switch (level) {
            case 5:
                cls = "text-red-900";
                extraClasses = "bg-warning text-warning-content";
                break;
            case 4:
                cls = "text-orange-900";
                extraClasses = "bg-orange-400 text-warning-content";
                break;
        }
        const result = /^(\[.+\]) (.+)$/gm.exec(message);
        let info: string;
        let newMsg: string;
        if (!result) {
            return;
        }
        info = result[1];
        newMsg = result[2];
        logElement.push({
            info,
            msg: newMsg,
            cls,
            extraClasses: extraClasses,
        });
        if (isAtBottom) {
            consoleEl!.scrollTop = consoleEl!.scrollHeight;
        }
    }
</script>

{@render children()}
