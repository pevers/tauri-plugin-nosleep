export declare enum NoSleepType {
    PreventUserIdleDisplaySleep = "PreventUserIdleDisplaySleep",
    PreventUserIdleSystemSleep = "PreventUserIdleSystemSleep"
}
export declare function block(noSleepType: NoSleepType): Promise<void>;
export declare function unblock(): Promise<void>;
