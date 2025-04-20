import type { PlayFolder } from '$bindings/play';
import type { CreatePlayFolderDto, CreatePlayFolderResponse } from '$bindings/play/dto';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_play_folders(): Promise<PlayFolder[]> {
  const folders = await invoke<PlayFolder[]>('get_play_folders');
  return folders;
}

export async function action_create_play_folder(data: CreatePlayFolderDto): Promise<CreatePlayFolderResponse> {
  const res = await invoke<CreatePlayFolderResponse>('create_play_folder', { dto: data });
  return res;
}
