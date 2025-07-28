import type { PlayItemKind } from '$bindings/constants';
import { invoke } from '@tauri-apps/api/core';

export async function action_delete_play_items(play_ids: string[]): Promise<Array<[string, PlayItemKind]>> {
  return await invoke<Array<[string, PlayItemKind]>>('delete_play_items', {
    playIds: play_ids,
  });
}

export async function action_update_play_item_name(play_id: string, name: string): Promise<void> {
  await invoke<void>('update_play_item_name', {
    playId: play_id,
    name,
  });
}
