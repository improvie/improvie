import type { RuleFormat } from '$lib/types/rules';
import { writable } from 'svelte/store';

export const current_track = writable<string | undefined>(undefined);

export interface CurrentRule {
  rules: RuleFormat[];
  idx: number;
}

export const current_rule = writable<CurrentRule | undefined>(undefined);
