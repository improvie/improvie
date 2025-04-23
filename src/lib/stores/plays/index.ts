import type { PlayFolderNode } from '$bindings/play';
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
  goto('/editor');
}

export const current_play_folder_ids: Writable<string[]> = writable([UUID_NIL]);

export const play_folder_nodes: SvelteMap<string, PlayFolderNode> = new SvelteMap();

export function init_play_items() {
  action_get_plays_hierarchy(UUID_NIL).then((v) => {
    v.forEach((obj) => {
      play_folder_nodes.set(obj[0], obj[1]);
    });
  }).catch((e) => {
    console.error(e);
  });

  action_get_playlists().then((v) => {
    v.forEach((obj) => {
      playlists.set(obj.id, obj);
    });
  }).catch((e) => {
    console.error(e);
  });

  action_get_play_folders().then((v) => {
    v.forEach((obj) => {
      play_folders.set(obj.id, obj);
    });
  }).catch((e) => {
    console.error(e);
  });
}
