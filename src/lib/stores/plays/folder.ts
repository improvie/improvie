import type { PlayFolder } from '$lib/types/plays';
import type { CreatePlayFolder } from '$lib/types/plays/create';
import type { Writable } from 'svelte/store';
import { action_create_play_folder } from '$lib/action/plays/folder';
import { writable } from 'svelte/store';
import { play_folder_nodes } from '.';

export const play_folders: Writable<Map<string, PlayFolder>> = writable(new Map());

export async function create_play_folder(data: CreatePlayFolder): Promise<void> {
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
