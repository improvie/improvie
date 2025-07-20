import type { Playlist } from '$bindings/play';
import type { CreatePlaylistRequest } from '$bindings/play/request';
import { action_delete_play_item, action_update_play_item_name } from '$lib/action/plays';
import { action_create_playlist } from '$lib/action/plays/playlists';
import { SvelteMap } from 'svelte/reactivity';
import { play_folder_nodes } from '.';

export const playlists: SvelteMap<string, Playlist> = new SvelteMap();

export async function delete_playlist(id: string): Promise<void> {
  const uids = await action_delete_play_item(id);
  for (const uid of uids) {
    playlists.delete(uid);
  }
}

export async function update_playlist_name(id: string, name: string): Promise<void> {
  await action_update_play_item_name(id, name);
  const p = playlists.get(id);
  if (p) {
    p.title = name;
    playlists.set(id, p);
  }
}

export async function create_playlist(data: CreatePlaylistRequest): Promise<void> {
  const res = await action_create_playlist(data);

  play_folder_nodes.set(res.folder_node.folder, res.folder_node);
  playlists.set(res.playlist.id, res.playlist);
}
