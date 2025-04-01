import type { Content } from '$lib/types/item';
import { writable } from 'svelte/store';

export const current_track = writable<Content | undefined>(undefined);
