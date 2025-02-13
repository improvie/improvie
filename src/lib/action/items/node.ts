import type { FolderNode } from '$lib/types/item';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_items_hierarchy(
  folder_id?: string,
): Promise<Map<string, FolderNode>> {
  const nodes = await invoke<object>('get_items_hierarchy', {
    folderId: folder_id,
  });
  const map = new Map(Object.entries(nodes));
  return map;
}
