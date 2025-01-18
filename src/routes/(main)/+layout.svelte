<script lang="ts">
    import Toaster from "$lib/components/toaster/Toaster.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { page } from "$app/state";
    let { children, data } = $props();

    let isRunning = $state(data.isRunning);
    let confirmModal: HTMLDialogElement;

    async function toggleRun() {
        isRunning = await invoke<boolean>("toggle_run");
    }

    listen<boolean>("toggle-bridge", (event) => {
        isRunning = event.payload;
    });
</script>

<Toaster />

<main>
    <div class="container py-12 mb-16">
        {@render children()}
    </div>

    <!-- Bottom Navigation -->
    <div class="btm-nav z-10">
        <div class="bg-base-300 max-w-fit px-2">
            <label
                class="label cursor-pointer flex flex-col justify-between gap-1"
            >
                <input
                    type="checkbox"
                    onclickcapture={(e) => {
                        if (isRunning) {
                            e.preventDefault();
                            confirmModal.showModal();
                        } else {
                            toggleRun();
                        }
                    }}
                    class="toggle toggle-accent"
                    bind:checked={isRunning}
                />
                <span class="label-text"
                    >Bridge Status: <span
                        class="font-semibold {isRunning
                            ? 'text-success'
                            : 'text-warning'}">{isRunning ? "On" : "Off"}</span
                    ></span
                >
            </label>
        </div>
        <a
            href="/console"
            class="transition-all {page.url.pathname === '/console'
                ? 'active'
                : ''}"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="m6.75 7.5 3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0 0 21 18V6a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 6v12a2.25 2.25 0 0 0 2.25 2.25Z"
                />
            </svg>
            <span class="btm-nav-label">Console</span>
        </a>
        <a
            href="/settings"
            class="transition-all {page.url.pathname === '/settings'
                ? 'active'
                : ''}"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
                />
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                />
            </svg>
            <span class="btm-nav-label">Settings</span>
        </a>
        <a
            href="/"
            class="transition-all {page.url.pathname === '/' ? 'active' : ''}"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z"
                />
            </svg>
            <span class="btm-nav-label">About</span>
        </a>
    </div>
</main>

<dialog bind:this={confirmModal} class="modal">
    <div class="modal-box">
        <h3 class="text-lg font-bold">Confirm</h3>
        <p class="py-4">
            Are you sure you want to stop the notification bridge?
        </p>
        <div class="modal-action gap-4">
            <button
                onclick={() => {
                    isRunning = false;
                    confirmModal.close();
                }}
                class="btn btn-error">Stop bridge</button
            >
            <button
                onclick={() => {
                    confirmModal.close();
                }}
                class="btn btn-primary">Cancel</button
            >
        </div>
    </div>
</dialog>
