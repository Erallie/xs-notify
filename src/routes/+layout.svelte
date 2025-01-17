<script lang="ts">
    import "../app.pcss";
    import { setContext } from "svelte";
    import type { Logs } from "$lib/types/types.js";
    import { attachLogger } from "@tauri-apps/plugin-log";

    let { children } = $props();

    // with TargetKind::Webview enabled this function will print logs to the browser console
</script>

{@render children()}
