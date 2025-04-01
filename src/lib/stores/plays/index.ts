import type { PlayFolderNode } from '$lib/types/plays';
import type { Writable } from 'svelte/store';
import { goto } from '$app/navigation';
import { action_get_play_folders } from '$lib/action/plays/folder';
import { action_get_plays_hierarchy } from '$lib/action/plays/node';
import { action_get_playlists } from '$lib/action/plays/playlists';
import { UUID_NIL } from '$lib/utils';
import { SvelteMap } from 'svelte/reactivity';
import { writable } from 'svelte/store';
import { play_folders } from './folder';
import { playlists } from './playlist';

export const current_playlist_id: Writable<string> = writable(UUID_NIL);

export function select_playlist(playlist_id: string) {
  current_playlist_id.set(playlist_id);
  goto('/playlist');
}

export const current_play_folder_ids: Writable<string[]> = writable([UUID_NIL]);

export const play_folder_nodes: Writable<SvelteMap<string, PlayFolderNode>> = writable(new SvelteMap());

export function init_play_items() {
  action_get_plays_hierarchy().then((v) => {
    play_folder_nodes.set(v);
  }).catch((e) => {
    console.error(e);
  });

  action_get_playlists().then((v) => {
    const b = v.map(obj => [obj.id, obj] as const);
    playlists.set(new SvelteMap(b));
  }).catch((e) => {
    console.error(e);
  });

  action_get_play_folders().then((v) => {
    const b = v.map(obj => [obj.id, obj] as const);
    play_folders.set(new SvelteMap(b));
  }).catch((e) => {
    console.error(e);
  });
}
