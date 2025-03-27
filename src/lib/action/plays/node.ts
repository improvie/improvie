import type { PlayFolderNode } from '$lib/types/plays';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_plays_hierarchy(
  folder_id?: string,
): Promise<Map<string, PlayFolderNode>> {
  const nodes = await invoke<object>('get_plays_hierarchy', {
    folderId: folder_id,
  });
  const map = new Map(Object.entries(nodes));
  return map;
}
