import type { Folder } from '$bindings/item';
import type { CreateFolderDto, CreateFolderResponse } from '$bindings/item/dto';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_folders(): Promise<Folder[]> {
  const folders = await invoke<Folder[]>('get_folders');
  return folders;
}

export async function action_create_folder(data: CreateFolderDto): Promise<CreateFolderResponse> {
  const res = await invoke<CreateFolderResponse>('create_folder', { dto: data });
  return res;
}
