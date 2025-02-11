import type { Content, Folder, FolderNode } from "$lib/types/item.ts";
import { UUID_NIL } from "$lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { type Writable, writable } from "svelte/store";

export const current_folder_ids: Writable<string[]> = writable([UUID_NIL]);

export const contents: Writable<Map<string, Content>> = writable(new Map());
export const folders: Writable<Map<string, Folder>> = writable(new Map());

export const folder_nodes: Writable<Map<string, FolderNode>> = writable(
  new Map(),
);

export function init_items() {
  invoke<object>("get_items_hierarchy").then((v) => {
    folder_nodes.set(new Map(Object.entries(v)));
  });

  invoke<Content[]>("get_contents").then((v) => {
    const b = v.map((obj) => [obj.id, obj] as const);
    contents.set(new Map(b));
  });
  invoke<Folder[]>("get_folders").then((v) => {
    const b = v.map((obj) => [obj.id, obj] as const);
    folders.set(new Map(b));
  });
}
