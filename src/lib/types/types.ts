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

    isWhitelist: boolean;
    appList: string[];

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
    warning?: boolean;
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

    isWhitelist,
    appList,

    autoLaunch,
    minimize,
    minimizeOnStart,
}

export type SettingSection = {
    title: string;
    description?: string;
    warning?: boolean;
    children: any;
}

export type Logs = {
    info: string;
    msg: string;
    infoCls: string;
    extraClasses?: string;
}