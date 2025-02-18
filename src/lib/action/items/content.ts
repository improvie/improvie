import type { Content } from '$lib/types/item';
import type { CreateContent } from '$lib/types/item/create';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_contents(): Promise<Content[]> {
  const contents = await invoke<Content[]>('get_contents');
  return contents;
}

export async function action_create_content(data: CreateContent): Promise<Content> {
  const content = await invoke<Content>('create_content', { dto: data });
  return content;
}
