import type { ChangeEventHandler, MouseEventHandler } from "svelte/elements";

export type XSNotifySettings = {
    autoRun: boolean;

    port: number;
    host: string;
    pollingRate: number;

    dynamicTimeout: boolean;
    defaultTimeout: number;

    readingSpeed: number;
    minTimeout: number;
    maxTimeout: number;

    skippedApps: string[];

    autoLaunch: boolean;
    minimize: boolean;
    minimizeOnStart: boolean;
}

export enum SettingType {
    toggle,
    string,
    number,
    stringArray
}

export type Setting = {
    title: string;
    description?: string;
    settings: XSNotifySettings;
    setting: WhichSetting;
    callback?: Function;
}

export enum WhichSetting {
    autoRun,
    port,
    host,
    pollingRate,
    dynamicTimeout,
    defaultTimeout,
    readingSpeed,
    minTimeout,
    maxTimeout,
    skippedApps,
    autoLaunch,
    minimize,
    minimizeOnStart,
}

export type DynamicSettings = {
    mainSetting: Setting;
    enabledSettings: Setting[];
    disabledSettings: Setting[];
}

// Type guard function
export function isDynamicSettings(obj: any): obj is DynamicSettings {
    return (
        typeof obj === 'object' &&
        obj !== null &&
        'mainSetting' in obj &&
        'enabledSettings' in obj &&
        'disabledSettings' in obj &&
        isSetting(obj.mainSetting) &&
        Array.isArray(obj.enabledSettings) &&
        obj.enabledSettings.every(isSetting) &&
        Array.isArray(obj.disabledSettings) &&
        obj.disabledSettings.every(isSetting)
    );
}

// Helper function to check if an object is of type Setting
export function isSetting(obj: any): obj is Setting {
    return (
        typeof obj === 'object' &&
        obj !== null &&
        'title' in obj &&
        (typeof obj.title === 'string') &&
        ('description' in obj ? typeof obj.description === 'string' : true) && // Check if description is a string if it exists
        'settings' in obj &&
        typeof obj.settings === 'object' && // Check if settings is an object
        'setting' in obj &&
        Object.values(WhichSetting).includes(obj.setting) && // Check if setting is a string
        ('callback' in obj ? typeof obj.callback === 'function' : true) // Check if description is a string if it exists
    );
}

export type SettingSection = {
    title: string;
    description?: string;
    settings: Setting[] | DynamicSettings;
}

export type Logs = {
    msg: string;
    level: number;
    extraClasses?: string;
}