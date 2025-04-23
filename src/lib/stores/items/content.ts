import type { Content } from '$bindings/item';
import type { CreateContentDto, CreateContentResponse } from '$bindings/item/dto';
import { action_delete_item, action_update_item_name } from '$lib/action/items';
import { action_create_content } from '$lib/action/items/content';
import { SvelteMap } from 'svelte/reactivity';
import { folder_nodes } from '.';

export const contents: SvelteMap<string, Content> = new SvelteMap();

export async function delete_content(id: string): Promise<void> {
  const uids = await action_delete_item(id);
  for (const uid of uids) {
    contents.delete(uid);
  }
}

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

export async function create_content(data: CreateContentDto): Promise<void> {
  const res = await action_create_content(data);
  update_content(res);
}
