import type { PlayFolderNode } from '$lib/types/plays';
import { invoke } from '@tauri-apps/api/core';
import { SvelteMap } from 'svelte/reactivity';

export async function action_get_plays_hierarchy(
  folder_id?: string,
): Promise<SvelteMap<string, PlayFolderNode>> {
  const nodes = await invoke<object>('get_plays_hierarchy', {
    folderId: folder_id,
  });
  const map = new SvelteMap(Object.entries(nodes));
  return map;
}
