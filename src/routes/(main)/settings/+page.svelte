<script lang="ts">
    import MinMaxSlider from "$lib/components/settings/MinMaxSlider.svelte";
    import MultiSelect from "$lib/components/settings/MultiSelect.svelte";
    import NumberInputBig from "$lib/components/settings/NumberInputBig.svelte";
    import SettingSection from "$lib/components/settings/SettingSection.svelte";
    import Switch from "$lib/components/settings/Switch.svelte";
    import { fly, slide } from "svelte/transition";

    let { data } = $props();

    let newSettings = $state(data.settings);

    let rangeSlider: number[] = $state([
        newSettings.minTimeout,
        newSettings.maxTimeout,
    ]);

    let optionsChanged: boolean = $derived(
        JSON.stringify(newSettings) !== JSON.stringify(data.settings),
    );

    $effect(() => {
        newSettings.minTimeout = rangeSlider[0];
        newSettings.maxTimeout = rangeSlider[1];
    });
</script>

<div class="container">
    <h1 class="text-center text-4xl font-extrabold text-nowrap tracking-wide">
        Settings
    </h1>
    <div class="my-12 space-y-16">
        <!-- General Settings -->
        <SettingSection
            title="General"
            description="General settings for XSNotify"
        >
            <Switch
                bind:checked={newSettings.autoRun}
                class="toggle-primary"
                label="Run on boot"
                description="Runs XSNotify on boot."
            />
            <NumberInputBig
                variant="primary"
                label="Polling rate"
                description="The rate which XSNotify will check for new notifications."
                bind:value={newSettings.pollingRate}
            />
            <MultiSelect bind:selected={newSettings.skippedApps} />
        </SettingSection>

        <!-- Dynamic Notifications Settings -->
        <SettingSection
            title="Notifications"
            description="Notification options for XSNotify"
        >
            <Switch
                label="Enable Dynamic Notifications"
                class="toggle-primary"
                bind:checked={newSettings.dynamicTimeout}
            />
            {#if newSettings.dynamicTimeout}
                <div
                    transition:slide
                    class="space-y-8 card card-bordered card-compact shadow-xl bg-neutral/40 text-neutral-content"
                >
                    <div class="card-body">
                        <NumberInputBig
                            variant="primary"
                            label="Reading speed"
                            description="Your reading speed in WPM (Words per minute) which will be used to calculate the time notifications will be shown for."
                            bind:value={newSettings.readingSpeed}
                        />
                        <div>
                            <MinMaxSlider
                                label="Notification time"
                                min={2}
                                max={300}
                                bind:values={rangeSlider}
                            />
                            <div class="flex flex-row justify-between label">
                                <span class="label-text text-pretty"
                                    >Minimum</span
                                >
                                <span class="label-text text-pretty"
                                    >Maximum</span
                                >
                            </div>
                        </div>
                    </div>
                </div>
            {:else}
                <div transition:slide>
                    <NumberInputBig
                        variant="primary"
                        label="Notification timeout"
                        description="The amount of time a notification will be shown."
                        bind:value={newSettings.defaultTimeout}
                    />
                </div>
            {/if}
        </SettingSection>

        <!-- XSOverlay Settings -->
        <SettingSection
            title="XSOverlay"
            description="Settings for XSNotify to connect to XSOverlay"
        >
            <div>
                <div class="text-lg font-semibold mb-2">Hostname</div>
                <input
                    bind:value={newSettings.host}
                    class="input input-primary"
                />
                <div class="label">
                    <span class="label-text text-pretty">
                        The IP or hostname that XSOverlay is running on.
                    </span>
                </div>
            </div>
            <NumberInputBig
                variant="primary"
                label="Port"
                description="The port XSOverlay is accessible on."
                bind:value={newSettings.port}
            />
        </SettingSection>
    </div>
</div>

<!-- Alert of changed options -->
{#if optionsChanged}
    <div out:fly={{ y: 20 }} class="toast toast-center mb-16 z-10">
        <div class="alert alert-info gap-6">
            <span
                >You need to restart the notification server to apply the new
                settings.</span
            >
            <button
                onclick={() => {
                    console.log("Restart action here.");
                }}
                class="btn btn-secondary btn-sm h-10">Apply & Restart</button
            >
        </div>
    </div>
{/if}
