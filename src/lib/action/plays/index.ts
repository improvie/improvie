import { invoke } from '@tauri-apps/api/core';

export async function action_delete_play_item(play_id: string): Promise<string[]> {
  return await invoke<string[]>('delete_play_item', {
    playId: play_id,
  });
}

export async function action_update_play_item_name(play_id: string, name: string): Promise<void> {
  await invoke<void>('update_play_item_name', {
    playId: play_id,
    name,
  });
}
