import type { Playlist } from '$bindings/play';
import type { CreatePlaylistDto } from '$bindings/play/dto';
import type { Writable } from 'svelte/store';
import { action_delete_play_item, action_update_play_item_name } from '$lib/action/plays';
import { action_create_playlist } from '$lib/action/plays/playlists';
import { SvelteMap } from 'svelte/reactivity';
import { writable } from 'svelte/store';
import { play_folder_nodes } from '.';

export const playlists: Writable<SvelteMap<string, Playlist>> = writable(new SvelteMap());

export async function delete_playlist(id: string): Promise<void> {
  const uids = await action_delete_play_item(id);
  playlists.update((v) => {
    for (const uid of uids) {
      v.delete(uid);
    }
    return v;
  });
}

export async function update_playlist_name(id: string, name: string): Promise<void> {
  await action_update_play_item_name(id, name);
  playlists.update((v) => {
    const p = v.get(id);
    if (p) {
      p.title = name;
      v.set(id, p);
    }
    return v;
  });
}

export async function create_playlist(data: CreatePlaylistDto): Promise<void> {
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
