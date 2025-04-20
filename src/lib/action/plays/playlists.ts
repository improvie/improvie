import type { Playlist } from '$bindings/play';
import type { CreatePlaylistDto, CreatePlaylistResponse } from '$bindings/play/dto';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_playlists(): Promise<Playlist[]> {
  const playlists = await invoke<Playlist[]>('get_playlists');
  return playlists;
}

export async function action_create_playlist(data: CreatePlaylistDto): Promise<CreatePlaylistResponse> {
  const res = await invoke<CreatePlaylistResponse>('create_playlist', { dto: data });
  return res;
}
