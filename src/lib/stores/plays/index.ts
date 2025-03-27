import type { PlayFolderNode } from '$lib/types/plays';
import type { Writable } from 'svelte/store';
import { action_get_play_folders } from '$lib/action/plays/folder';
import { action_get_plays_hierarchy } from '$lib/action/plays/node';
import { action_get_playlists } from '$lib/action/plays/playlists';
import { UUID_NIL } from '$lib/utils';
import { writable } from 'svelte/store';
import { play_folders } from './folder';
import { playlists } from './playlist';

export const current_play_folder_ids: Writable<string[]> = writable([UUID_NIL]);

export const play_folder_nodes: Writable<Map<string, PlayFolderNode>> = writable(new Map());

export function init_play_items() {
  action_get_plays_hierarchy().then((v) => {
    play_folder_nodes.set(v);
  });

  action_get_playlists().then((v) => {
    const b = v.map(obj => [obj.id, obj] as const);
    playlists.set(new Map(b));
  });

  action_get_play_folders().then((v) => {
    const b = v.map(obj => [obj.id, obj] as const);
    play_folders.set(new Map(b));
  });
}
