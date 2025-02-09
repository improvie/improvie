import type { Content, Folder, FolderNode } from "$lib/types/item.ts";
import { invoke } from "@tauri-apps/api/core";
import { type Writable, writable } from "svelte/store";

export const contents: Writable<Map<string, Content>> = writable(new Map());
export const folders: Writable<Map<string, Folder>> = writable(new Map());

export const folder_nodes: Writable<Map<string, FolderNode>> = writable(
  new Map(),
);

export function init_items() {
  invoke<Map<string, FolderNode>>("get_items_hierarchy").then((v) =>
    folder_nodes.set(v),
  );

  invoke<Map<string, Content>>("get_contents").then((v) => contents.set(v));
  invoke<Map<string, Folder>>("get_folders").then((v) => folders.set(v));
}
