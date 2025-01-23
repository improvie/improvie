import type { FolderNode } from "$lib/types/item.ts";
import { writable, type Writable } from "svelte/store";

export const items: Writable<FolderNode> = writable();
