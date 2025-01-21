<script lang="ts">
    import { invalidate, invalidateAll } from "$app/navigation";
    import MinMaxSlider from "$lib/components/settings/MinMaxSlider.svelte";
    import MultiSelect from "$lib/components/settings/MultiSelect.svelte";
    import NumberInputBig from "$lib/components/settings/NumberInputBig.svelte";
    import SettingSection from "$lib/components/settings/SettingSection.svelte";
    import Switch from "$lib/components/settings/Switch.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { fly, slide } from "svelte/transition";

    let { data } = $props();

    // let oldSettings = $state(data.settings);

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

    async function onChange() {
        let isRunning = await invoke<boolean>("get_running");
        if (!isRunning) {
            await invoke("update_settings", { settings: newSettings });
        }
        invalidateAll();
    }
</script>

<div class="container">
    <h1 class="text-center text-4xl font-extrabold text-nowrap tracking-wide">
        Settings
    </h1>
    <div class="my-12 space-y-16">
        <!-- General Settings -->
        <SettingSection
            title="General"
            description="General settings for XS Notify"
        >
            <Switch
                bind:checked={newSettings.autoRun}
                class="toggle-primary"
                label="Auto-start Bridge"
                description="Start the notification bridge on application launch."
                onchange={onChange}
            />
            <NumberInputBig
                variant="primary"
                label="Polling Rate"
                description="The rate in milliseconds at which which XSNotify will check for new notifications."
                bind:value={newSettings.pollingRate}
                changeAmount={10}
                onchange={onChange}
            />
            <Switch
                bind:checked={newSettings.isWhitelist}
                class="toggle-primary"
                label="Use Whitelist"
                description="Toggle this on if you want to treat the below setting as a whitelist instead of a blacklist."
                onchange={onChange}
            />
            <MultiSelect
                label="{newSettings.isWhitelist
                    ? 'Whitelisted'
                    : 'Blacklisted'} applications"
                bind:selected={newSettings.appList}
                description="Apps that XS Notify {newSettings.isWhitelist
                    ? 'will'
                    : 'will not'} push notifications for."
                onchange={onChange}
            />
        </SettingSection>

        <!-- Dynamic Notifications Settings -->
        <SettingSection
            title="Notifications"
            description="Notification options for XS Notify"
        >
            <Switch
                label="Dynamic Notifications"
                class="toggle-primary"
                bind:checked={newSettings.dynamicTimeout}
                description="Dynamically sets the notification display time based on the length of the notification text."
                onchange={onChange}
            />
            {#if newSettings.dynamicTimeout}
                <div
                    transition:slide
                    class="space-y-8 card card-bordered card-compact shadow-xl bg-neutral/40 text-neutral-content"
                >
                    <div class="card-body">
                        <NumberInputBig
                            variant="primary"
                            label="Reading Speed"
                            description="Your reading speed in words per minute (WPM), which will be used to calculate the amount of time notifications will be shown for."
                            bind:value={newSettings.readingSpeed}
                            changeAmount={15}
                            onchange={onChange}
                        />
                        <div>
                            <MinMaxSlider
                                label="Min/Max Display Time"
                                description="The minimum and maximum amount of time in seconds that notifications will be shown for."
                                min={2}
                                max={300}
                                bind:values={rangeSlider}
                                onchange={onChange}
                            />
                        </div>
                    </div>
                </div>
            {:else}
                <div transition:slide>
                    <NumberInputBig
                        variant="primary"
                        label="Display Time"
                        description="The amount of time a notification will be shown for in seconds."
                        bind:value={newSettings.defaultTimeout}
                        changeAmount={1}
                        onchange={onChange}
                    />
                </div>
            {/if}
        </SettingSection>

        <!-- XSOverlay Settings -->
        <SettingSection
            title="XSOverlay"
            description="Settings for XS Notify to connect to XSOverlay"
        >
            <div>
                <div class="text-lg font-semibold mb-2">Hostname</div>
                <input
                    bind:value={newSettings.host}
                    class="input input-primary"
                    onchange={onChange}
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
                changeAmount={10}
                onchange={onChange}
            />
        </SettingSection>
        <SettingSection
            title="Startup"
            description="Launch settings for XS Notify"
        >
            <Switch
                bind:checked={newSettings.autoLaunch}
                class="toggle-primary"
                label="Auto-launch"
                description="Launch XS Notify on system startup."
                onchange={onChange}
            />
            <Switch
                bind:checked={newSettings.minimize}
                class="toggle-primary"
                label="Minimize to Tray"
                description="Minimize to the system tray instead of closing."
                onchange={onChange}
            />
            <Switch
                bind:checked={newSettings.minimizeOnStart}
                class="toggle-primary"
                label="Start Minimized"
                description="Minimize XS Notify to the system tray when it launches."
                onchange={onChange}
            />
        </SettingSection>
    </div>
</div>

<!-- Alert of changed options -->
{#if optionsChanged && data.isRunning}
    <div out:fly={{ y: 20 }} class="toast toast-center mb-16 z-10">
        <div class="alert alert-info gap-6 bg-secondary text-secondary-content">
            <span
                >You must restart the notification bridge to apply the new
                settings.</span
            >
            <button
                onclick={async () => {
                    await invoke("update_settings", { settings: newSettings });
                    invalidateAll();
                }}
                class="btn btn-primary text-primary-content btn-sm h-10"
                >Apply & Restart</button
            >
        </div>
    </div>
{/if}
