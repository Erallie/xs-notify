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

export type SettingSection = {
    title: string;
    description?: string;
    children: any;
}

export type Logs = {
    info: string;
    msg: string;
    infoCls: string;
    extraClasses?: string;
}