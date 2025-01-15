<script lang="ts">
    import { WhichSetting } from "$lib/types/types";
    import Setting from "../../Setting.svelte";
    import SettingSection from "../../SettingSection.svelte";

    let { data } = $props();
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
                settings={[
                    {
                        title: "Auto-run",
                        description:
                            "Runs the notification bridge on application launch.",
                        settings: settings,
                        setting: WhichSetting.autoRun,
                    },
                    {
                        title: "Polling Rate",
                        description:
                            "The rate at which XS Notify checks for new notifications, specified in milliseconds.",
                        settings: settings,
                        setting: WhichSetting.pollingRate,
                    },
                    {
                        title: "Skipped apps",
                        description:
                            "A list of names of apps that XS Notify will not push notifications for.",
                        settings: settings,
                        setting: WhichSetting.skippedApps,
                    },
                ]}
            />
            <!-- Dynamic Notifications Settings -->
            <SettingSection
                title="Dynamic Notifications"
                description="Dynamically sets the notification display time based on the length of the notification text."
                settings={{
                    mainSetting: {
                        title: "Enable Dynamic Notifications",
                        settings: settings,
                        setting: WhichSetting.dynamicTimeout,
                    },
                    enabledSettings: [
                        {
                            title: "Reading Speed",
                            description:
                                "Your reading speed in words per minute (WPM), which will be used to calculate the display time for dynamic notifications.",
                            settings: settings,
                            setting: WhichSetting.readingSpeed,
                        },
                        {
                            title: "Minimum notification time",
                            description:
                                "The minimum amount of time a notification will be displayed, in seconds.",
                            settings: settings,
                            setting: WhichSetting.minTimeout,
                        },
                        {
                            title: "Maximum notification time",
                            description:
                                "The maximum amount of time a notification will be displayed, in seconds.",
                            settings: settings,
                            setting: WhichSetting.maxTimeout,
                        },
                    ],
                    disabledSettings: [
                        {
                            title: "Notification time",
                            description:
                                "The amount of time for which a notification will be displayed, in seconds",
                            settings: settings,
                            setting: WhichSetting.defaultTimeout,
                        },
                    ],
                }}
            />

            <!-- XSOverlay Settings -->
            <SettingSection
                title="XSOverlay"
                settings={[
                    {
                        title: "Port",
                        description: "The port that XSOverlay is listening on.",
                        settings: settings,
                        setting: WhichSetting.port,
                    },
                    {
                        title: "Host",
                        description:
                            "The hostname that XSOverlay is listening on.",
                        settings: settings,
                        setting: WhichSetting.host,
                    },
                ]}
            />

            <SettingSection
                title="Startup"
                description="Options relating to the startup of XS Notify."
                settings={[
                    {
                        title: "Auto-launch",
                        description: "Launch XS Notify on system startup.",
                        settings: settings,
                        setting: WhichSetting.autoLaunch,
                    },
                    {
                        title: "Minimize to Tray",
                        description:
                            "Minimize to the system tray instead of closing.",
                        settings: settings,
                        setting: WhichSetting.minimize,
                    },
                    {
                        title: "Start Minimized",
                        description:
                            "Minimize XS Notify to the system tray when it launches.",
                        settings: settings,
                        setting: WhichSetting.minimizeOnStart,
                    },
                ]}
            />
        </div>
    {/await}
</section>
