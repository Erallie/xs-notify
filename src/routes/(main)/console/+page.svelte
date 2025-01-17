<script lang="ts">
    import type { Logs } from "$lib/types/types.js";
    import { attachLogger } from "@tauri-apps/plugin-log";
    import { getContext } from "svelte";

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
        const result = /^\[[^\[\]]+\](\[[^\[\]]+\])\[.+\] (.+)$/gm.exec(
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

<section class="container">
    <h1
        class="text-center text-4xl font-extrabold text-nowrap tracking-wide mb-12"
    >
        Console
    </h1>
    <div class="mockup-code overflow-y-auto" id="console-el">
        {#each logElement as log}
            <pre class="text-wrap -indent-12 pl-12 {log.extraClasses}"><code
                    ><span class={log.infoCls}>{log.info}</span> {log.msg}</code
                ></pre>
        {/each}
    </div>
</section>
