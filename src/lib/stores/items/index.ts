import type { FolderNode } from '$bindings/item';
import type { Writable } from 'svelte/store';
import { action_get_contents } from '$lib/action/items/content';
import { action_get_folders } from '$lib/action/items/folder';
import { action_get_items_hierarchy } from '$lib/action/items/node';
import { ULID_NIL } from '$lib/utils';
import { SvelteMap } from 'svelte/reactivity';
import { writable } from 'svelte/store';
import { contents } from './content';
import { folders } from './folder';

export const current_folder_ids: Writable<string[]> = writable([ULID_NIL]);

export const folder_nodes: SvelteMap<string, FolderNode> = new SvelteMap();

export function init_items() {
  action_get_items_hierarchy(ULID_NIL).then((v) => {
    v.forEach((obj) => {
      folder_nodes.set(obj[0], obj[1]);
    });
  }).catch((e) => {
    console.error(e);
  });

  action_get_contents().then((v) => {
    v.forEach((obj) => {
      contents.set(obj.id, obj);
    });
  }).catch((e) => {
    console.error(e);
  });

  action_get_folders().then((v) => {
    v.forEach((obj) => {
      folders.set(obj.id, obj);
    });
  }).catch((e) => {
    console.error(e);
  });
}
