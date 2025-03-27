import type { Playlist } from '$lib/types/plays';
import type { CreatePlaylist, CreatePlaylistResponse } from '$lib/types/plays/create';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_playlists(): Promise<Playlist[]> {
  const contents = await invoke<Playlist[]>('get_playlists');
  return contents;
}

export async function action_create_playlist(data: CreatePlaylist): Promise<CreatePlaylistResponse> {
  const res = await invoke<CreatePlaylistResponse>('create_playlist', { dto: data });
  return res;
}
