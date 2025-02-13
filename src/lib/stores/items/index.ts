import { action_get_contents } from '$lib/action/items/content';
import { action_get_folders } from '$lib/action/items/folder';
import { action_get_items_hierarchy } from '$lib/action/items/node';
import type { FolderNode } from '$lib/types/item/index.ts';
import { UUID_NIL } from '$lib/utils';
import { type Writable, writable } from 'svelte/store';
import { contents } from './content';
import { folders } from './folder';

export const current_folder_ids: Writable<string[]> = writable([UUID_NIL]);

export const folder_nodes: Writable<Map<string, FolderNode>> = writable(new Map());

export function init_items() {
  action_get_items_hierarchy().then((v) => {
    folder_nodes.set(v);
  });

  action_get_contents().then((v) => {
    const b = v.map((obj) => [obj.id, obj] as const);
    contents.set(new Map(b));
  });

  action_get_folders().then((v) => {
    const b = v.map((obj) => [obj.id, obj] as const);
    folders.set(new Map(b));
  });
}
