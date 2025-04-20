import type { Content } from '$bindings/item';
import type { CreateContentDto, CreateContentResponse } from '$bindings/item/dto';
import type { Writable } from 'svelte/store';
import { action_delete_item, action_update_item_name } from '$lib/action/items';
import { action_create_content } from '$lib/action/items/content';
import { SvelteMap } from 'svelte/reactivity';
import { writable } from 'svelte/store';
import { folder_nodes } from '.';

export const contents: Writable<SvelteMap<string, Content>> = writable(new SvelteMap());

export async function delete_content(id: string): Promise<void> {
  const uids = await action_delete_item(id);
  contents.update((v) => {
    for (const uid of uids) {
      v.delete(uid);
    }
    return v;
  });
}

export async function update_content_name(id: string, name: string): Promise<void> {
  await action_update_item_name(id, name);
  contents.update((v) => {
    const c = v.get(id);
    if (c) {
      c.title = name;
      v.set(id, c);
    }
    return v;
  });
}

export function update_content(res: CreateContentResponse): void {
  folder_nodes.update((v) => {
    v.set(res.folder_node.folder, res.folder_node);
    return v;
  });

  contents.update((v) => {
    v.set(res.content.id, res.content);
    return v;
  });
}

export async function create_content(data: CreateContentDto): Promise<void> {
  const res = await action_create_content(data);
  update_content(res);
}
