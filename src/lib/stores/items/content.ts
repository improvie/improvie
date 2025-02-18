import type { Content } from '$lib/types/item';
import type { CreateContent } from '$lib/types/item/create';
import type { Writable } from 'svelte/store';
import { action_create_content } from '$lib/action/items/content';
import { writable } from 'svelte/store';

export const contents: Writable<Map<string, Content>> = writable(new Map());

export async function create_content(data: CreateContent): Promise<Content> {
  const content = await action_create_content(data);

  contents.update((v) => {
    v.set(content.id, content);
    return v;
  });

  return content;
}
