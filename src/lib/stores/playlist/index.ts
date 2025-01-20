import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Playlist } from "$lib/types/playlist";

// TODO: playlists

export const playlists: Writable<Playlist[]> = writable([]);

export async function addPlaylist(id: string) {
  await invoke("add_playlist", { id });
}

export async function removePlaylist(id: string) {
  await invoke("remove_playlist", { id });
}
