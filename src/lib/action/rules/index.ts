import type { RuleFormat, RuleType } from '$bindings/rule';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_rules(playlist_id: string): Promise<RuleType[]> {
  const rules = await invoke<RuleType[]>('get_rules', { playlistId: playlist_id });
  return rules;
}

export async function action_update_rules(playlist_id: string, rules: RuleType[]): Promise<void> {
  await invoke('update_rules', {
    playlistId: playlist_id,
    rules,
  });
}

export async function action_get_rules_format(playlist_id: string): Promise<RuleFormat[]> {
  const format = await invoke<RuleFormat[]>('get_rules_format', { playlist_id });
  return format;
}

export async function action_get_rules_format_with_shuffle(playlist_id: string): Promise<RuleFormat[]> {
  const format = await invoke<RuleFormat[]>('get_rules_format_with_shuffle', { playlist_id });
  return format;
}

export async function action_get_thumbnail_content_uid(playlist_id: string): Promise<string | undefined> {
  return await invoke<string | undefined>('get_thumbnail_content_uid', { playlist_id });
}
