import type { PlayFolderNode } from '$bindings/play';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_plays_hierarchy(
  folder_id: string,
): Promise<[string, PlayFolderNode][]> {
  const nodes = await invoke<object>('get_plays_hierarchy', {
    folderId: folder_id,
  });
  return Object.entries(nodes);
}
