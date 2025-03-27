import type { Playlist } from '$lib/types/plays';
import type { CreatePlaylist } from '$lib/types/plays/create';
import type { Writable } from 'svelte/store';
import { action_create_playlist } from '$lib/action/plays/playlists';
import { writable } from 'svelte/store';
import { play_folder_nodes } from '.';

export const playlists: Writable<Map<string, Playlist>> = writable(new Map());

export async function create_playlist(data: CreatePlaylist): Promise<void> {
  const res = await action_create_playlist(data);

  play_folder_nodes.update((v) => {
    v.set(res.folder_node.folder, res.folder_node);
    return v;
  });

  playlists.update((v) => {
    v.set(res.playlist.id, res.playlist);
    return v;
  });
}
