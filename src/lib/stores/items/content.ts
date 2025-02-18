import type { Content } from '$lib/types/item';
import type { CreateContent } from '$lib/types/item/create';
import type { Writable } from 'svelte/store';
import { action_create_content } from '$lib/action/items/content';
import { writable } from 'svelte/store';
import { folder_nodes } from '.';

export const contents: Writable<Map<string, Content>> = writable(new Map());

export async function create_content(data: CreateContent): Promise<void> {
  const res = await action_create_content(data);

  folder_nodes.update((v) => {
    v.set(res.folder_node.folder, res.folder_node);
    return v;
  });

  contents.update((v) => {
    v.set(res.content.id, res.content);
    return v;
  });
}
