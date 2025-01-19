import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export const favorite: Writable<string[]> = writable([]);

export async function addFavoritePlaylist(id: string) {
  await invoke("add_favorite_playlist", { id });
  favorite.update((ids) => [...ids, id]);
}

export async function removeFavoritePlaylist(id: string) {
  await invoke("remove_favorite_playlist", { id });
  favorite.update((ids) => ids.filter((i) => i !== id));
}

// TODO: use this
export async function moveFavoritePlaylist(id: string, index: number) {
  await invoke("move_favorite_playlist", { id, sort_order: index });
  favorite.update((ids) => {
    ids.splice(ids.indexOf(id), 1);
    ids.splice(index, 0, id);
    return ids;
  });
}
