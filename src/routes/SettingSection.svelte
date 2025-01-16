<script lang="ts">
    import {
        type SettingSection,
        isDynamicSettings,
        isSetting,
    } from "$lib/types/types";
    import Setting from "./Setting.svelte";

    let { title, description, settings }: SettingSection = $props();

    if (isDynamicSettings(settings)) {
    }
    function setDynamicSetting() {
        if (isDynamicSettings(settings)) {
            return settings.mainSetting.settings.dynamicTimeout;
        }
    }
    let showEnabled = $state(setDynamicSetting());
    function toggleDynamicSetting(value: boolean) {
        showEnabled = value;
    }
</script>

<div class="my-12">
    <h2 class="divider text-xl font-bold">{title}</h2>
    {#if description && description != ""}
        <p>
            {description}
        </p>
    {/if}
    <div class="py-4 space-y-8">
        {#if Array.isArray(settings)}
            {#each settings as setting}
                {#if isSetting(setting)}
                    <Setting
                        title={setting.title}
                        description={setting.description}
                        settings={setting.settings}
                        setting={setting.setting}
                    />
                {:else}
                    <p class="text-error">
                        <code>{settings}</code> is not a valid setting!
                    </p>
                {/if}
            {/each}
        {:else if isDynamicSettings(settings)}
            <Setting
                title={settings.mainSetting.title}
                description={settings.mainSetting.description}
                settings={settings.mainSetting.settings}
                setting={settings.mainSetting.setting}
                callback={toggleDynamicSetting}
            />
            {#if showEnabled}
                {#each settings.enabledSettings as setting}
                    <Setting
                        title={setting.title}
                        description={setting.description}
                        settings={setting.settings}
                        setting={setting.setting}
                    />
                {/each}
            {:else if !showEnabled}
                {#each settings.disabledSettings as setting}
                    <Setting
                        title={setting.title}
                        description={setting.description}
                        settings={setting.settings}
                        setting={setting.setting}
                    />
                {/each}
            {/if}
        {/if}
    </div>
</div>
