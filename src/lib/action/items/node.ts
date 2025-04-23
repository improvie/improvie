import type { FolderNode } from '$bindings/item';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_items_hierarchy(
  folder_id: string,
): Promise<[string, FolderNode][]> {
  const nodes = await invoke<object>('get_items_hierarchy', {
    folderId: folder_id,
  });
  return Object.entries(nodes);
}
