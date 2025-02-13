import type { Folder } from '$lib/types/item';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_folders(): Promise<Folder[]> {
  const folders = await invoke<Folder[]>('get_folders');
  return folders;
}
