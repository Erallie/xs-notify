<script lang="ts">
    import type { Logs } from "$lib/types/types.js";
    import { attachLogger } from "@tauri-apps/plugin-log";
    import { openUrl } from "@tauri-apps/plugin-opener";

    let { data } = $props();

    let logElement: Logs[] = $state(data.logs);

    // let logElement: Logs[] = $state([]);

    let detachLogger: () => void;

    $effect(() => {
        let consoleEl = document.getElementById("console-el");
        if (consoleEl) {
            consoleEl.scrollTop = consoleEl.scrollHeight;
        }

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

    function splitMessage(message: string): string[] {
        return message.split(/(https?:\/\/[^\s]+)/g);
    }

    function isUrl(value: string): boolean {
        return /^https?:\/\/[^\s]+$/.test(value);
    }

    async function openLogLink(url: string) {
        try {
            await openUrl(url);
        } catch (error) {
            console.error("Failed to open link:", error);
        }
    }

    function addToConsole(message: string, level: number) {
        let consoleEl = document.getElementById("console-el");

        let isAtBottom = false;
        if (
            consoleEl &&
            consoleEl.scrollTop >=
                consoleEl.scrollHeight - consoleEl.clientHeight * 1.1
        ) {
            isAtBottom = true;
        }
        let infoCls = "text-gray-500";
        let extraClasses;
        switch (level) {
            case 5: //error
                infoCls = "text-red-900";
                extraClasses = "bg-warning text-warning-content";
                break;
            case 4: //warn
                infoCls = "text-orange-900";
                extraClasses = "bg-orange-400 text-warning-content";
                break;
        }
        const result = /^\[[^\[\]]+\](\[[^\[\]]+\])\[.+\] ([^\[]+)$/gm.exec(
            message,
        );
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
            infoCls,
            extraClasses: extraClasses,
        });
        if (isAtBottom) {
            consoleEl!.scrollTop = consoleEl!.scrollHeight;
        }
    }
</script>

<section id="console-container" class="container">
    <h1
        class="text-center text-4xl font-extrabold text-nowrap tracking-wide mb-12"
    >
        Console
    </h1>
    <div id="console-el" class="mockup-code overflow-y-auto">
        {#each logElement as log}
            <pre class="text-wrap -indent-12 pl-12 {log.extraClasses}"><code
                ><span class={log.infoCls}>{log.info}</span> {#each splitMessage(log.msg) as part}{#if isUrl(part)}<button type="button" class="log-link" onclick={() => openLogLink(part)}>{part}</button>{:else}{part}{/if}{/each}</code
                ></pre>
        {/each}
    </div>
</section>

<style>
    .log-link {
        display: inline;
        padding: 0;
        border: none;
        background: none;
        color: inherit;
        font: inherit;
        text-decoration: underline;
        cursor: pointer;
    }

    .log-link:hover {
        opacity: 0.8;
    }
</style>