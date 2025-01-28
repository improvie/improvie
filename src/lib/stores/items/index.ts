import type { Content, Folder, FolderNode } from "$lib/types/item.ts";
import { type Writable, writable } from "svelte/store";

export const contents: Writable<Map<string, Content>> = writable(new Map());
export const folders: Writable<Map<string, Folder>> = writable(new Map());

export const folder_nodes: Writable<Map<string, FolderNode>> = writable(
  new Map(),
);
