import type { PlayFolder } from '$lib/types/plays';
import type { CreatePlayFolder, CreatePlayFolderResponse } from '$lib/types/plays/create';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_play_folders(): Promise<PlayFolder[]> {
  const folders = await invoke<PlayFolder[]>('get_play_folders');
  return folders;
}

export async function action_create_play_folder(data: CreatePlayFolder): Promise<CreatePlayFolderResponse> {
  const res = await invoke<CreatePlayFolderResponse>('create_play_folder', { dto: data });
  return res;
}
