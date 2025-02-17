import type { Playlist, PlaylistFolder } from '$lib/types/playlist.ts';
import type { Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { get, writable } from 'svelte/store';

// TODO: playlists

export const playlists: Writable<Map<string, Playlist>> = writable(new Map());
export const playlist_folders: Writable<Map<string, PlaylistFolder>> = writable(new Map());

export function getPlaylist(id: string): Playlist | undefined {
  return get(playlists)
    .values()
    .find(v => v.id === id);
}

export async function addPlaylist(id: string) {
  await invoke('add_playlist', { id });
}

export async function removePlaylist(id: string) {
  await invoke('remove_playlist', { id });
}
