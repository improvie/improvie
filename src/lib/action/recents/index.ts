import { invoke } from '@tauri-apps/api/core';

export async function action_update_content_by_used(
  content_id: string,
): Promise<void> {
  await invoke('update_content_by_used', {
    contentId: content_id,
  });
}
export async function action_update_playlist_by_used(
  playlist_id: string,
): Promise<void> {
  await invoke('update_playlist_by_used', {
    playlistId: playlist_id,
  });
}

export async function action_get_recent_contents(
  limit: number | undefined,
): Promise<string[]> {
  return await invoke<string[]>('get_recent_contents', { limit });
}

export async function action_get_recent_playlists(
  limit: number | undefined,
): Promise<string[]> {
  return await invoke<string[]>('get_recent_playlists', { limit });
}
