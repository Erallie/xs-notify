<script lang="ts">
    import { WhichSetting } from "$lib/types/types";
    import Setting from "../../Setting.svelte";
    import SettingSection from "../../SettingSection.svelte";

    let { data } = $props();

    let showEnabled = $state(data.settings.dynamicTimeout);
    function toggleDynamicSetting(value: boolean) {
        showEnabled = value;
    }
</script>

<section class="container">
    <h1 class="text-center text-4xl font-extrabold text-nowrap tracking-wide">
        Settings
    </h1>
    {#await data.settings}
        loading settings... <!-- replace with something to display while settings are loading -->
    {:then settings}
        <div class="my-12">
            <!-- General Settings -->
            <SettingSection
                title="General"
                description="General settings for XS Notify."
                ><Setting
                    title="Auto-run"
                    description="Runs the notification bridge on application launch."
                    {settings}
                    setting={WhichSetting.autoRun}
                /><Setting
                    title="Polling Rate"
                    description="The rate at which XS Notify checks for new notifications, specified in milliseconds."
                    {settings}
                    setting={WhichSetting.pollingRate}
                /><Setting
                    title="Skipped apps"
                    description="A list of names of apps that XS Notify will not push notifications for."
                    {settings}
                    setting={WhichSetting.skippedApps}
                /></SettingSection
            >
            <!-- Dynamic Notifications Settings -->
            <SettingSection
                title="Dynamic Notifications"
                description="Dynamically sets the notification display time based on the length of the notification text."
            >
                <Setting
                    title="Enable Dynamic Notifications"
                    {settings}
                    setting={WhichSetting.dynamicTimeout}
                    callback={toggleDynamicSetting}
                />
                {#if showEnabled}
                    <Setting
                        title="Reading Speed"
                        description="Your reading speed in words per minute (WPM), which will be used to calculate the display time for dynamic notifications."
                        {settings}
                        setting={WhichSetting.readingSpeed}
                    />
                    <Setting
                        title="Minimum notification time"
                        description="The minimum amount of time a notification will be displayed, in seconds."
                        {settings}
                        setting={WhichSetting.minTimeout}
                    />
                    <Setting
                        title="Maximum notification time"
                        description="The maximum amount of time a notification will be displayed, in seconds."
                        {settings}
                        setting={WhichSetting.maxTimeout}
                    />
                {:else if !showEnabled}
                    <Setting
                        title="Notification time"
                        description="The amount of time for which a notification will be displayed, in seconds"
                        {settings}
                        setting={WhichSetting.defaultTimeout}
                    />
                {/if}
            </SettingSection>

            <!-- XSOverlay Settings -->
            <SettingSection title="XSOverlay">
                <Setting
                    title="Port"
                    description="The port that XSOverlay is listening on."
                    {settings}
                    setting={WhichSetting.port}
                />
                <Setting
                    title="Host"
                    description="The hostname that XSOverlay is listening on."
                    {settings}
                    setting={WhichSetting.host}
                />
            </SettingSection>

            <SettingSection
                title="Startup"
                description="Options relating to the startup of XS Notify."
            >
                <Setting
                    title="Auto-launch"
                    description="Launch XS Notify on system startup."
                    {settings}
                    setting={WhichSetting.autoLaunch}
                />
                <Setting
                    title="Minimize to Tray"
                    description="Minimize to the system tray instead of closing."
                    {settings}
                    setting={WhichSetting.minimize}
                />
                <Setting
                    title="Start Minimized"
                    description="Minimize XS Notify to the system tray when it launches."
                    {settings}
                    setting={WhichSetting.minimizeOnStart}
                />
            </SettingSection>
        </div>
    {/await}
</section>
