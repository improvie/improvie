import { writable } from 'svelte/store';

export const current_track = writable<string | undefined>(undefined);
