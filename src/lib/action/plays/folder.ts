import type { PlayFolder } from '$bindings/play';
import type { CreatePlayFolderRequest } from '$bindings/play/request';
import type { CreatePlayFolderResponse } from '$bindings/play/response';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_play_folders(): Promise<PlayFolder[]> {
  const folders = await invoke<PlayFolder[]>('get_play_folders');
  return folders;
}

export async function action_create_play_folder(
  data: CreatePlayFolderRequest,
): Promise<CreatePlayFolderResponse> {
  const res = await invoke<CreatePlayFolderResponse>('create_play_folder', { request: data });
  return res;
}
