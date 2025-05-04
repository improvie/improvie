import { invoke } from '@tauri-apps/api/core';

export async function action_select_folder_dialog(): Promise<string> {
  return await invoke<string>('open_select_folder_dialog');
}
