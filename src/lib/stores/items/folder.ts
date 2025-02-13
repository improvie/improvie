import type { Folder } from '$lib/types/item';
import type { Writable } from 'svelte/store';
import { writable } from 'svelte/store';

export const folders: Writable<Map<string, Folder>> = writable(new Map());
