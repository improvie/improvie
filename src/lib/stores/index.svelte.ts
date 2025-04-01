import type { Snippet } from 'svelte';
import { getContext, setContext } from 'svelte';

const key = Symbol('layout-slots');

export interface LayoutSlots {
  header: Snippet | undefined;
}

export function initSlots(): LayoutSlots {
  const slots = $state<LayoutSlots>({ header: undefined });
  return setContext(key, slots);
}

export function setSlots(slots: LayoutSlots) {
  const context: LayoutSlots = getContext(key);
  Object.assign(context, slots);
}
