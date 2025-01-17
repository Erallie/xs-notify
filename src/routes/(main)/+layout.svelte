<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { setContext } from "svelte";
    import type { Logs } from "$lib/types/types.js";
    import { attachLogger } from "@tauri-apps/plugin-log";

    let { children, data } = $props();

    let isRunning = $state(data.isRunning);

    let toggleRunText = $state(setRunText());

    function setRunText() {
        switch (isRunning) {
            case true:
                return "Stop Bridge";
            case false:
                return "Start Bridge";
        }
    }

    async function toggleRun() {
        isRunning = await invoke<boolean>("toggle_run");
        toggleRunText = setRunText();
    }

    listen<boolean>("toggle-bridge", (event) => {
        isRunning = event.payload;
        toggleRunText = setRunText();
        // console.error("testing");
        // console.log(`isRunning = ${isRunning}`);
    });
</script>

<main class="drawer lg:drawer-open">
    <input id="my-drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col items-center justify-center my-12">
        <!-- Page content here -->

        <label
            for="my-drawer"
            class="btn drawer-button lg:hidden fixed top-6 left-8 btn-neutral"
        >
            <svg
                height="21"
                width="21"
                viewBox="0 0 21 21"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g
                    stroke="currentColor"
                    stroke-width="4"
                    stroke-linecap="round"
                >
                    <line x1="2" y1="10" x2="19" y2="10" />
                    <line x1="2" y1="2" x2="19" y2="2" />
                    <line x1="2" y1="19" x2="19" y2="19" />
                </g>
                =
            </svg>
        </label>
        {@render children()}
    </div>
    <div class="drawer-side">
        <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"
        ></label>
        <nav class="menu bg-base-200 text-base-content min-h-full w-80 p-4">
            <!-- Sidebar content here -->
            <ul>
                <li><a href="/">About</a></li>
                <li>
                    <a href="/settings" aria-label="close sidebar">Settings</a>
                </li>
                <li>
                    <a href="/console">Console</a>
                </li>
                <li>
                    <button onclick={toggleRun}>{toggleRunText}</button>
                </li>
            </ul>
        </nav>
    </div>
</main>
