import type { Content } from '$bindings/item';
import type { CreateContentRequest } from '$bindings/item/command';
import type { CreateContentResponse } from '$bindings/item/response';
import { action_update_item_name } from '$lib/action/items';
import { action_create_content } from '$lib/action/items/content';
import { SvelteMap } from 'svelte/reactivity';
import { folder_nodes } from '.';

export const contents: SvelteMap<string, Content> = new SvelteMap();

export async function update_content_name(id: string, name: string): Promise<void> {
  await action_update_item_name(id, name);
  const c = contents.get(id);
  if (c) {
    c.title = name;
    contents.set(id, c);
  }
}

export function update_content(res: CreateContentResponse): void {
  folder_nodes.set(res.folder_node.folder, res.folder_node);

  contents.set(res.content.id, res.content);
}

export async function create_content(data: CreateContentRequest): Promise<void> {
  try {
    const res = await action_create_content(data);
    update_content(res);
  }
  catch (error) {
    console.error('Error creating content:', error);
    throw error; // Re-throw the error for further handling if needed
  }
}
