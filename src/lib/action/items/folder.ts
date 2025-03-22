import type { Folder } from '$lib/types/item';
import type { CreateFolder, CreateFolderResponse } from '$lib/types/item/create';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_folders(): Promise<Folder[]> {
  const folders = await invoke<Folder[]>('get_folders');
  return folders;
}

export async function action_create_folder(data: CreateFolder): Promise<CreateFolderResponse> {
  const res = await invoke<CreateFolderResponse>('create_folder', { dto: data });
  return res;
}
