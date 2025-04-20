import type { FolderNode } from '$bindings/item';
import { invoke } from '@tauri-apps/api/core';
import { SvelteMap } from 'svelte/reactivity';

export async function action_get_items_hierarchy(
  folder_id: string,
): Promise<SvelteMap<string, FolderNode>> {
  const nodes = await invoke<object>('get_items_hierarchy', {
    folderId: folder_id,
  });
  const map = new SvelteMap(Object.entries(nodes));
  return map;
}
