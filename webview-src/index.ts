import { invoke } from '@tauri-apps/api/core'


export async function prevent_display_sleep(): Promise<void> {
  return await invoke('plugin:nosleep|prevent_display_sleep');
}

export async function prevent_system_sleep(): Promise<void> {
  return await invoke('plugin:nosleep|prevent_system_sleep');
}

export async function unblock(): Promise<void> {
  return await invoke('plugin:nosleep|unblock');
}