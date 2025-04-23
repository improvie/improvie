import type { Folder } from '$bindings/item';
import type { CreateFolderDto } from '$bindings/item/dto';
import { action_delete_item, action_update_item_name } from '$lib/action/items';
import { action_create_folder } from '$lib/action/items/folder';
import { SvelteMap } from 'svelte/reactivity';
import { folder_nodes } from '.';

export const folders: SvelteMap<string, Folder> = new SvelteMap();

export async function delete_folder(id: string): Promise<void> {
  const uids = await action_delete_item(id);
  for (const uid of uids) {
    folders.delete(uid);
  }
}

export async function update_folder_name(id: string, name: string): Promise<void> {
  await action_update_item_name(id, name);
  const f = folders.get(id);
  if (f) {
    f.title = name;
    folders.set(id, f);
  }
}

export async function create_folder(data: CreateFolderDto): Promise<void> {
  const res = await action_create_folder(data);

  folder_nodes.set(res.folder_node.folder, res.folder_node);
  folders.set(res.folder.id, res.folder);
}
