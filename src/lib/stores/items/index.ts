import type { FolderNode } from "$lib/types/item";
import { writable, type Writable } from "svelte/store";

export const items: Writable<FolderNode> = writable();
