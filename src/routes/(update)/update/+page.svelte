<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    async function closeWindow() {
        await getCurrentWindow().close();
    }

    async function openLink() {
        await invoke("open_update_link");
    }

    async function update() {
        await invoke("download_update");
    }
    let updateFailed = $state(false);

    let downloadLink = $state("");

    listen<string>("update-failed", (event) => {
        updateFailed = true;
        downloadLink = event.payload;
    });
</script>

<section class="text-center relative h-dvh">
    <div class="absolute top-1/2 -translate-y-1/2 w-full space-y-6">
        {#if !updateFailed}
            <h1 class="text-3xl font-extrabold text-nowrap">
                <span class="tracking-wide"
                    >A <span class="text-purple-600">New Version</span></span
                ><br />is Available
            </h1>

            <p class="text-pretty -mb-2">Would you like to install it now?</p>
            <div class="space-x-4">
                <button class="btn btn-secondary" onclick={update}
                    >Download & install</button
                >
                <!-- <button class="btn btn-neutral" onclick={openLink}>Open link</button
            > --><button
                    class="btn btn-neutral"
                    onclick={closeWindow}>Not now</button
                >
            </div>
        {:else}
            <h1 class="text-2xl font-extrabold">
                There Was an <span class="text-purple-600">Error</span><br
                /><span class="text-lg">downloading the new version.</span>
            </h1>

            <p class="text-pretty -mb-2">
                Would you like to download it manually from GitHub instead?
            </p>
            <div class="space-x-4">
                <button class="btn btn-secondary" onclick={openLink}
                    >Open link</button
                >
                <button class="btn btn-neutral" onclick={closeWindow}
                    >Not now</button
                >
            </div>
        {/if}
    </div>
</section>
