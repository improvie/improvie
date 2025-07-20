import type { Content } from '$bindings/item';
import type { CreateContentResponse } from '$bindings/item/dto';
import type { CreateContentRequest } from '$bindings/item/request';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_contents(): Promise<Content[]> {
  const contents = await invoke<Content[]>('get_contents');
  return contents;
}

export async function action_create_content(data: CreateContentRequest): Promise<CreateContentResponse> {
  const res = await invoke<CreateContentResponse>('create_content', { request: data });
  return res;
}
