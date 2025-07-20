import type { Folder } from '$bindings/item';
import type { CreateFolderRequest } from '$bindings/item/request';
import type { CreateFolderResponse } from '$bindings/item/response';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_folders(): Promise<Folder[]> {
  const folders = await invoke<Folder[]>('get_folders');
  return folders;
}

export async function action_create_folder(data: CreateFolderRequest): Promise<CreateFolderResponse> {
  const res = await invoke<CreateFolderResponse>('create_folder', { request: data });
  return res;
}
