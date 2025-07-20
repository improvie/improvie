import type { Playlist } from '$bindings/play';
import type { CreatePlaylistRequest } from '$bindings/play/request';
import type { CreatePlaylistResponse } from '$bindings/play/response';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_playlists(): Promise<Playlist[]> {
  const playlists = await invoke<Playlist[]>('get_playlists');
  return playlists;
}

export async function action_create_playlist(
  data: CreatePlaylistRequest,
): Promise<CreatePlaylistResponse> {
  const res = await invoke<CreatePlaylistResponse>('create_playlist', { request: data });
  return res;
}
