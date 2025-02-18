import type { ContentFileDialogResponse, ImageFileDialogResponse } from '$lib/types/dialog';
import { invoke } from '@tauri-apps/api/core';

export async function action_select_content_dialog(): Promise<ContentFileDialogResponse> {
  return await invoke<ContentFileDialogResponse>('open_select_content_dialog');
}

export async function action_select_thumbnail_dialog(): Promise<ImageFileDialogResponse> {
  return await invoke<ImageFileDialogResponse>('open_select_thumbnail_dialog');
}
