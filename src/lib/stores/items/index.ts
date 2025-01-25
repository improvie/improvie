import type { Content, Folder } from "$lib/types/item.ts";
import { writable, type Writable } from "svelte/store";

export const contents: Writable<Content[]> = writable([]);
export const folders: Writable<Folder[]> = writable([]);
