import type { CreateContentResponse } from '$lib/types/item/create';
import { invoke } from '@tauri-apps/api/core';

export async function action_delete_item(item_id: string): Promise<string[]> {
  return await invoke<string[]>('delete_item', {
    itemId: item_id,
  });
}

export async function action_update_item_name(item_id: string, name: string): Promise<void> {
  await invoke<void>('update_item_name', {
    itemId: item_id,
    name,
  });
}

export async function action_import_youtube_video(parent_folder_id: string, url: string): Promise<CreateContentResponse> {
  return await invoke<CreateContentResponse>('import_youtube_video', {
    parentFolderId: parent_folder_id,
    url,
  });
}
