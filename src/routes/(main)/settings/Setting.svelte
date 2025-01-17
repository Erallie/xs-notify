<script lang="ts">
    import {
        type Setting,
        SettingType,
        WhichSetting,
        type XSNotifySettings,
    } from "$lib/types/types";
    import { invoke } from "@tauri-apps/api/core";

    let {
        title,
        description,
        settings,
        setting,
        callback = $bindable(),
    }: Setting = $props();

    let type: SettingType;

    function returnValue(setting: WhichSetting) {
        switch (setting) {
            case WhichSetting.autoRun:
                type = SettingType.toggle;
                return settings.autoRun;

            case WhichSetting.port:
                type = SettingType.number;
                return settings.port;

            case WhichSetting.host:
                type = SettingType.string;
                return settings.host;

            case WhichSetting.pollingRate:
                type = SettingType.number;
                return settings.pollingRate;

            case WhichSetting.dynamicTimeout:
                type = SettingType.toggle;
                return settings.dynamicTimeout;

            case WhichSetting.defaultTimeout:
                type = SettingType.number;
                return settings.defaultTimeout;

            case WhichSetting.readingSpeed:
                type = SettingType.number;
                return settings.readingSpeed;

            case WhichSetting.minTimeout:
                type = SettingType.number;
                return settings.minTimeout;

            case WhichSetting.maxTimeout:
                type = SettingType.number;
                return settings.maxTimeout;

            case WhichSetting.isWhitelist:
                type = SettingType.toggle;
                return settings.isWhitelist;

            case WhichSetting.appList:
                type = SettingType.stringArray;
                return settings.appList;

            case WhichSetting.autoLaunch:
                type = SettingType.toggle;
                return settings.autoLaunch;

            case WhichSetting.minimize:
                type = SettingType.toggle;
                return settings.minimize;

            case WhichSetting.minimizeOnStart:
                type = SettingType.toggle;
                return settings.minimizeOnStart;
        }
    }

    let value = $state(returnValue(setting));

    let currentApp = $state("");

    function updateSettings(input: any) {
        let theseSettings: XSNotifySettings = settings;
        if (input instanceof Event) {
            let target = input.target;
            if (target instanceof HTMLInputElement) {
                switch (type) {
                    case SettingType.toggle:
                        value = target.checked;
                        break;
                    case SettingType.string:
                        value = target.value;
                        break;
                    case SettingType.number:
                        value = target.valueAsNumber;
                        break;
                    default:
                }
                switch (setting) {
                    case WhichSetting.autoRun:
                        theseSettings.autoRun = value as boolean;
                        break;
                    case WhichSetting.port:
                        theseSettings.port = value as number;
                        break;
                    case WhichSetting.host:
                        theseSettings.host = value as string;
                        break;
                    case WhichSetting.pollingRate:
                        theseSettings.pollingRate = value as number;
                        break;
                    case WhichSetting.dynamicTimeout:
                        theseSettings.dynamicTimeout = value as boolean;
                        break;
                    case WhichSetting.defaultTimeout:
                        theseSettings.defaultTimeout = value as number;
                        break;
                    case WhichSetting.readingSpeed:
                        theseSettings.readingSpeed = value as number;
                        break;
                    case WhichSetting.minTimeout:
                        theseSettings.minTimeout = value as number;
                        break;
                    case WhichSetting.maxTimeout:
                        theseSettings.maxTimeout = value as number;
                        break;
                    case WhichSetting.autoLaunch:
                        theseSettings.autoLaunch = value as boolean;
                        break;
                    case WhichSetting.minimize:
                        theseSettings.minimize = value as boolean;
                        break;
                    case WhichSetting.minimizeOnStart:
                        theseSettings.minimizeOnStart = value as boolean;
                        break;
                    case WhichSetting.isWhitelist:
                        theseSettings.isWhitelist = value as boolean;
                        break;
                    default:
                }
            } else {
                console.log("Not the appropriate target: " + target);
                return;
            }
        } else if (
            Array.isArray(input) &&
            input.every((item) => typeof item === "string") &&
            setting == WhichSetting.appList
        ) {
            theseSettings.appList = input;
        }
        setDynamicSection();
        invoke("update_settings", { settings: theseSettings });
    }

    function setDynamicSection() {
        if (callback) {
            callback(value);
        }
    }

    function addApp() {
        if (
            Array.isArray(value) &&
            value.every((item) => typeof item === "string")
        ) {
            let thisValue = value;
            if (currentApp && !value.includes(currentApp)) {
                thisValue.push(currentApp);
            }
            updateSettings(thisValue);
            currentApp = "";
        }
    }

    function removeApp(app: string) {
        if (
            Array.isArray(value) &&
            value.every((item) => typeof item === "string")
        ) {
            value = value.filter((thisName: string) => {
                return thisName != app;
            });
        }
        updateSettings(value);
    }

    function setCurrentApp(event: Event) {
        event.preventDefault(); // Prevent the default form submission
        let target = event.target;
        if (target instanceof HTMLInputElement) {
            currentApp = target.value;
        } else {
            console.error("Target is not an HTMLInputElement!");
        }
    }
</script>

<div class="setting">
    <div class="text-lg font-semibold mb-2">
        {title}
    </div>
    {#if type == SettingType.number}
        <input
            type="number"
            class="input input-secondary"
            onchange={updateSettings}
            bind:value
        />
    {/if}
    {#if type == SettingType.string}
        <input
            type="text"
            class="input input-secondary"
            onchange={updateSettings}
            bind:value
        />
    {/if}
    {#if type == SettingType.toggle}
        <input
            type="checkbox"
            class="toggle toggle-secondary"
            onload={setDynamicSection}
            onchange={updateSettings}
            bind:checked={value as boolean}
        />
    {/if}
    {#if type == SettingType.stringArray}
        <form class="join" onsubmit={addApp}>
            <input
                type="text"
                class="input input-secondary join-item"
                bind:value={currentApp}
                onchange={setCurrentApp}
            />
            <button type="submit" class="btn btn-square btn-outline join-item">
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
                        <line x1="10" y1="2" x2="10" y2="19" />
                    </g>
                    +
                </svg>
            </button>
        </form>
    {/if}
    {#if description && description != ""}
        <div class="label">
            <span class="label-text text-pretty">{description}</span>
        </div>
    {/if}
    {#if type == SettingType.stringArray && Array.isArray(value) && value.every((item) => typeof item === "string")}
        <div class="mt-4">
            {#each value as string}
                <div class="join mr-2">
                    <div
                        class="btn btn-outline badge-lg join-item hover:no-animation"
                    >
                        {string}
                    </div>
                    <button
                        class="size-7 join-item btn btn-outline btn-warning"
                        onclick={() => removeApp(string)}
                    >
                        <svg
                            viewBox="0 0 20 20"
                            height="20"
                            width="20"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <g
                                stroke="currentColor"
                                stroke-width="4"
                                stroke-linecap="round"
                            >
                                <line x1="2" y1="2" x2="18" y2="18" />
                                <line x1="18" y1="2" x2="2" y2="18" />
                            </g>
                            X
                        </svg>
                    </button>
                </div>
            {/each}
        </div>
    {/if}
</div>
