import { type Writable, writable } from "svelte/store";

export type MainMenuType = "home" | "items" | "playlists";

export const main_menu_store: Writable<MainMenuType> = writable("home");
