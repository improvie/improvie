import type { Content } from '$bindings/item';
import type { Playlist } from '$bindings/play';
import type { CreatePlaylistRequest } from '$bindings/play/request';
import { action_update_play_item_name } from '$lib/action/plays';
import { action_create_playlist } from '$lib/action/plays/playlists';
import { action_get_thumbnail_content_uid } from '$lib/action/rules';
import { SvelteMap } from 'svelte/reactivity';
import { play_folder_nodes } from '.';

export const playlists: SvelteMap<string, Playlist> = new SvelteMap();

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

export async function get_playlist_thumbnail_path(playlist: Playlist, contents: SvelteMap<string, Content>): Promise<string | undefined> {
  if (playlist.thumbnail_path) {
    return playlist.thumbnail_path;
  }
  const content_id = await action_get_thumbnail_content_uid(playlist.id);
  if (content_id === undefined) {
    return undefined;
  }
  const content = contents.get(content_id);
  if (content === undefined) {
    return undefined;
  }
  return content.thumbnail_path ? content.thumbnail_path : undefined;
}
