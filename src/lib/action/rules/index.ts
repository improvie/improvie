import type { RuleFormat, RuleType } from '$bindings/rule';
import { invoke } from '@tauri-apps/api/core';

export async function action_get_rules(playlistId: string): Promise<RuleType[]> {
  const rules = await invoke<RuleType[]>('get_rules', { playlistId });
  return rules;
}

export async function action_update_rules(playlistId: string, rules: RuleType[]): Promise<void> {
  await invoke('update_rules', {
    playlistId,
    rules,
  });
}

export async function action_get_rules_format(playlistId: string): Promise<RuleFormat[]> {
  const format = await invoke<RuleFormat[]>('get_rules_format', { playlistId });
  return format;
}

export async function action_get_rules_format_with_shuffle(playlistId: string): Promise<RuleFormat[]> {
  const format = await invoke<RuleFormat[]>('get_rules_format_with_shuffle', { playlistId });
  return format;
}

export async function action_get_thumbnail_content_uid(playlistId: string): Promise<string | undefined> {
  return await invoke<string | undefined>('get_thumbnail_content_uid', { playlistId });
}
