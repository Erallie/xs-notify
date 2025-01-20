<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { setContext } from "svelte";
    import type { Logs } from "$lib/types/types.js";
    import { attachLogger } from "@tauri-apps/plugin-log";

    let { children, data } = $props();

    let isRunning = $state(data.isRunning);

    async function toggleRun() {
        isRunning = await invoke<boolean>("toggle_run");
    }

    listen<boolean>("toggle-bridge", (event) => {
        isRunning = event.payload;
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
        <nav class="menu bg-base-200 text-base-content min-h-full w-52 p-4">
            <!-- Sidebar content here -->
            <ul>
                <li><a href="/">About</a></li>
                <li>
                    <a href="/settings" aria-label="close sidebar">Settings</a>
                </li>
                <li>
                    <a href="/console">Console</a>
                </li>
            </ul>

            <button
                id="bridge-button"
                class="btn {isRunning
                    ? 'bg-warning text-warning-content hover:bg-warning hover:text-warning-content'
                    : 'bg-success text-success-content hover:bg-success hover:text-success-content'} absolute bottom-10 self-center"
                onclick={toggleRun}
                >{isRunning ? "Stop" : "Start"} Bridge</button
            >
        </nav>
    </div>
</main>
