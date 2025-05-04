import { invoke } from '@tauri-apps/api/core';

export async function action_select_folder_dialog(): Promise<string | null> {
  return await invoke<string | null>('open_select_folder_dialog');
}
