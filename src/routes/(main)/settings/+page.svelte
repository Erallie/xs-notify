<script lang="ts">
    import { WhichSetting } from "$lib/types/types";
    import Setting from "./Setting.svelte";
    import SettingSection from "./SettingSection.svelte";

    let { data } = $props();

    let showEnabled = $state(data.settings.dynamicTimeout);
    function toggleDynamicSetting(value: boolean) {
        showEnabled = value;
    }

    let listTitle = $state(returnListTitle(data.settings.isWhitelist));
    function returnListTitle(value: boolean) {
        switch (value) {
            case true:
                return "App Whitelist";
            case false:
                return "App Blacklist";
        }
    }

    let listDesc = $state(returnListDesc(data.settings.isWhitelist));

    function returnListDesc(value: boolean) {
        switch (value) {
            case true:
                return "will";
            case false:
                return "will not";
        }
    }

    function setListTitleAndDesc(value: boolean) {
        listTitle = returnListTitle(value);
        listDesc = returnListDesc(value);
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
            <SettingSection title="General" warning={true}
                ><Setting
                    title="Auto-run"
                    description="Runs the notification bridge on application launch."
                    {settings}
                    setting={WhichSetting.autoRun}
                /><Setting
                    title="Polling Rate"
                    description="The rate at which XS Notify checks for new notifications, specified in milliseconds."
                    warning={true}
                    {settings}
                    setting={WhichSetting.pollingRate}
                />
                <Setting
                    title="Use App Whitelist"
                    description="Toggle this on if you want to treat the below setting as a whitelist instead of a blacklist."
                    warning={true}
                    {settings}
                    setting={WhichSetting.isWhitelist}
                    callback={setListTitleAndDesc}
                />
                <Setting
                    title={listTitle}
                    description="A list of names of apps that XS Notify <strong>{listDesc}</strong> push notifications for."
                    warning={true}
                    {settings}
                    setting={WhichSetting.appList}
                /></SettingSection
            >
            <!-- Dynamic Notifications Settings -->
            <SettingSection title="Notifications" warning={true}>
                <Setting
                    title="Dynamic Notifications"
                    description="Dynamically sets the notification display time based on the length of the notification text."
                    warning={true}
                    {settings}
                    setting={WhichSetting.dynamicTimeout}
                    callback={toggleDynamicSetting}
                />
                {#if showEnabled}
                    <Setting
                        title="Reading Speed"
                        description="Your reading speed in words per minute (WPM), which will be used to calculate the display time for dynamic notifications."
                        warning={true}
                        {settings}
                        setting={WhichSetting.readingSpeed}
                    />
                    <Setting
                        title="Minimum notification time"
                        description="The minimum amount of time a notification will be displayed, in seconds."
                        warning={true}
                        {settings}
                        setting={WhichSetting.minTimeout}
                    />
                    <Setting
                        title="Maximum notification time"
                        description="The maximum amount of time a notification will be displayed, in seconds."
                        warning={true}
                        {settings}
                        setting={WhichSetting.maxTimeout}
                    />
                {:else if !showEnabled}
                    <Setting
                        title="Notification time"
                        description="The amount of time for which a notification will be displayed, in seconds"
                        warning={true}
                        {settings}
                        setting={WhichSetting.defaultTimeout}
                    />
                {/if}
            </SettingSection>

            <!-- XSOverlay Settings -->
            <SettingSection title="Bridge Connection" warning={true}>
                <Setting
                    title="Port"
                    description="The port that XSOverlay is listening on."
                    warning={true}
                    {settings}
                    setting={WhichSetting.port}
                />
                <Setting
                    title="Host"
                    description="The hostname that XSOverlay is listening on."
                    warning={true}
                    {settings}
                    setting={WhichSetting.host}
                />
            </SettingSection>

            <SettingSection title="Startup">
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
