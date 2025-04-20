import type { PlayFolder } from '$bindings/play';
import type { CreatePlayFolderDto } from '$bindings/play/dto';
import type { Writable } from 'svelte/store';
import { action_delete_play_item, action_update_play_item_name } from '$lib/action/plays';
import { action_create_play_folder } from '$lib/action/plays/folder';
import { SvelteMap } from 'svelte/reactivity';
import { writable } from 'svelte/store';
import { play_folder_nodes } from '.';

export const play_folders: Writable<SvelteMap<string, PlayFolder>> = writable(new SvelteMap());

export async function delete_play_folder(id: string): Promise<void> {
  const uids = await action_delete_play_item(id);
  play_folders.update((v) => {
    for (const uid of uids) {
      v.delete(uid);
    }
    return v;
  });
}

export async function update_play_folder_name(id: string, name: string): Promise<void> {
  await action_update_play_item_name(id, name);
  play_folders.update((v) => {
    const f = v.get(id);
    if (f) {
      f.title = name;
      v.set(id, f);
    }
    return v;
  });
}

export async function create_play_folder(data: CreatePlayFolderDto): Promise<void> {
  const res = await action_create_play_folder(data);

  play_folder_nodes.update((v) => {
    v.set(res.folder_node.folder, res.folder_node);
    return v;
  });

  play_folders.update((v) => {
    v.set(res.folder.id, res.folder);
    return v;
  });
}
