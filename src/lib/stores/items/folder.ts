import type { Folder } from '$bindings/item';
import type { CreateFolderDto } from '$bindings/item/dto';
import type { Writable } from 'svelte/store';
import { action_delete_item, action_update_item_name } from '$lib/action/items';
import { action_create_folder } from '$lib/action/items/folder';
import { SvelteMap } from 'svelte/reactivity';
import { writable } from 'svelte/store';
import { folder_nodes } from '.';

export const folders: Writable<SvelteMap<string, Folder>> = writable(new SvelteMap());

export async function delete_folder(id: string): Promise<void> {
  const uids = await action_delete_item(id);
  folders.update((v) => {
    for (const uid of uids) {
      v.delete(uid);
    }
    return v;
  });
}

export async function update_folder_name(id: string, name: string): Promise<void> {
  await action_update_item_name(id, name);
  folders.update((v) => {
    const f = v.get(id);
    if (f) {
      f.title = name;
      v.set(id, f);
    }
    return v;
  });
}

export async function create_folder(data: CreateFolderDto): Promise<void> {
  const res = await action_create_folder(data);

  folder_nodes.update((v) => {
    v.set(res.folder_node.folder, res.folder_node);
    return v;
  });

  folders.update((v) => {
    v.set(res.folder.id, res.folder);
    return v;
  });
}
