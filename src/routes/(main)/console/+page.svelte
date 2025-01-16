<script lang="ts">
    import type { Logs } from "$lib/types/types.js";
    let { data } = $props();
    import { getContext } from "svelte";

    const logElement: Logs[] = getContext("logElement");

    // console.log(getContext("logElement"));
    $effect(() => {
        let consoleEl = document.getElementById("console-el");
        if (consoleEl) {
            consoleEl.scrollTop = consoleEl.scrollHeight;
        }
    });
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
                    ><span class={log.cls}>{log.info}</span> {log.msg}</code
                ></pre>
        {/each}
    </div>
</section>
