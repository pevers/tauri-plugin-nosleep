import { invoke } from '@tauri-apps/api/tauri'

export enum NoSleepType {
  // Prevents the display from dimming automatically.
  // For example: playing a video.
  PreventUserIdleDisplaySleep = "PreventUserIdleDisplaySleep",

  // Prevents the system from sleeping automatically due to a lack of user activity.
  // For example: downloading a file in the background.
  PreventUserIdleSystemSleep = "PreventUserIdleSystemSleep"
};

export async function block(noSleepType: NoSleepType): Promise<void> {
  return await invoke('plugin:nosleep|block', {
    noSleepType
  });
}

export async function unblock(): Promise<void> {
  return await invoke('plugin:nosleep|unblock');
}