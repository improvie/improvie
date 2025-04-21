import type { RuleFormat, RuleType } from '$bindings/Rule';
import { action_get_rules_format } from '$lib/action/rules';
import { writable } from 'svelte/store';

export const current_track_id = writable<string | undefined>(undefined);

export interface CurrentRule {
  rules: RuleFormat[];
  idx: number;
}

export const current_rules = writable<RuleType[] | undefined>(undefined);
export const current_rule_formats = writable<CurrentRule | undefined>(undefined);

// $current_rules = rules; を一緒に実行してください
export async function set_current_rules(rules: RuleType[]) {
  const res = await action_get_rules_format(rules);
  if (res.length === 0) {
    clear_track();
    return;
  }
  current_rule_formats.set(
    {
      rules: res,
      idx: 0,
    },
  );
  current_track_id.set(res[0].content_id);
}

export function clear_track() {
  current_rules.set(undefined);
  current_rule_formats.set(undefined);
  current_track_id.set(undefined);
}
