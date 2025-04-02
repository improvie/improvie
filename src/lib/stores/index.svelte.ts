import type { Snippet } from 'svelte';
import { page } from '$app/state';
import { getContext, setContext } from 'svelte';

const key = Symbol('layout-slots');

export interface LayoutSlots {
  prefix_pathname: string;
  header: Snippet | undefined;
}

export function initSlots(): LayoutSlots {
  const slots = $state<LayoutSlots>({ prefix_pathname: '/', header: undefined });
  $effect(() => {
    console.log('initSlots', page.url.pathname);
    if (!page.url.pathname.startsWith(slots.prefix_pathname)) {
      setSlots({ prefix_pathname: page.url.pathname, header: undefined });
    }
  });
  return setContext(key, slots);
}

export function setSlots(slots: LayoutSlots) {
  const context: LayoutSlots = getContext(key);
  Object.assign(context, slots);
}
