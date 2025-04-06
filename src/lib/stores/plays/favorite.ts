import type { Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { writable } from 'svelte/store';

export const favoritePlaylists: Writable<string[]> = writable([]);

export async function initFavoritePlaylist() {
  const ids = await invoke<string[]>('get_favorite_playlists');
  favoritePlaylists.set(ids);
}

export async function addFavoritePlaylist(id: string) {
  await invoke('add_favorite_playlist', { id });
  favoritePlaylists.update(ids => [...ids, id]);
}

export async function removeFavoritePlaylist(id: string) {
  await invoke('remove_favorite_playlist', { id });
  favoritePlaylists.update(ids => ids.filter(i => i !== id));
}

// TODO: use this and impl on rust
export async function moveFavoritePlaylist(id: string, index: number) {
  await invoke('move_favorite_playlist', { id, sort_order: index });
  favoritePlaylists.update((ids) => {
    ids.splice(ids.indexOf(id), 1);
    ids.splice(index, 0, id);
    return ids;
  });
}
