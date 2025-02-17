import type { FileDialogResponse } from '$lib/types/dialog';
import { invoke } from '@tauri-apps/api/core';

export async function action_select_content_dialog(): Promise<FileDialogResponse> {
  return await invoke<FileDialogResponse>('open_select_content_dialog');
}

export async function action_select_thumbnail_dialog(): Promise<FileDialogResponse> {
  return await invoke<FileDialogResponse>('open_select_thumbnail_dialog');
}
