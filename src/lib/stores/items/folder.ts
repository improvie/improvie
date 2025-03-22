import type { Folder } from '$lib/types/item';
import type { CreateFolder } from '$lib/types/item/create';
import type { Writable } from 'svelte/store';
import { action_create_folder } from '$lib/action/items/folder';
import { writable } from 'svelte/store';
import { folder_nodes } from '.';

export const folders: Writable<Map<string, Folder>> = writable(new Map());

export async function create_folder(data: CreateFolder): Promise<void> {
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
