import type { AppSettings } from '$bindings/settings';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_app_settings(): Promise<AppSettings> {
  return await invoke<AppSettings>('get_app_settings');
}

export async function action_set_app_settings(settings: AppSettings): Promise<void> {
  return await invoke('set_app_settings', { settings });
}
