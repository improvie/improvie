import type { ItemKind } from '$bindings/constants';
import { invoke } from '@tauri-apps/api/core';

export async function action_delete_items(item_ids: string[]): Promise<Array<[string, ItemKind]>> {
  return await invoke<Array<[string, ItemKind]>>('delete_items', {
    itemIds: item_ids,
  });
}

export async function action_update_item_name(item_id: string, name: string): Promise<void> {
  await invoke<void>('update_item_name', {
    itemId: item_id,
    name,
  });
}
