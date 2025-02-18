import type { Content, ItemNode } from '$lib/types/item';
import type { CreateContent } from '$lib/types/item/create';
import type { Writable } from 'svelte/store';
import { action_create_content } from '$lib/action/items/content';
import { writable } from 'svelte/store';
import { folder_nodes } from '.';

export const contents: Writable<Map<string, Content>> = writable(new Map());

export async function create_content(data: CreateContent): Promise<Content> {
  const content = await action_create_content(data);

  folder_nodes.update((v) => {
    const new_node: ItemNode = { type: 'Content', id: content.id, sort_order: data.sort_order };

    const node = v.get(data.parent_folder_id);
    if (node === undefined) {
      v.set(data.parent_folder_id, { folder: data.parent_folder_id, items: [new_node] });
    }
    else {
      node.items.push(new_node);
    }
    return v;
  });

  contents.update((v) => {
    v.set(content.id, content);
    return v;
  });

  return content;
}
