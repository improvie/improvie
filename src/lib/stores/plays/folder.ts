import type { PlayFolder } from '$bindings/play';
import type { CreatePlayFolderRequest } from '$bindings/play/request';
import { action_delete_play_item, action_update_play_item_name } from '$lib/action/plays';
import { action_create_play_folder } from '$lib/action/plays/folder';
import { SvelteMap } from 'svelte/reactivity';
import { play_folder_nodes } from '.';

export const play_folders: SvelteMap<string, PlayFolder> = new SvelteMap();

export async function delete_play_folder(id: string): Promise<void> {
  const uids = await action_delete_play_item(id);
  for (const uid of uids) {
    play_folders.delete(uid);
  }
}

export async function update_play_folder_name(id: string, name: string): Promise<void> {
  await action_update_play_item_name(id, name);
  const f = play_folders.get(id);
  if (f) {
    f.title = name;
    play_folders.set(id, f);
  }
}

export async function create_play_folder(data: CreatePlayFolderRequest): Promise<void> {
  const res = await action_create_play_folder(data);

  play_folder_nodes.set(res.folder_node.folder, res.folder_node);
  play_folders.set(res.folder.id, res.folder);
}
