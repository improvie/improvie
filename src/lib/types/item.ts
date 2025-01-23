export interface Item {
  readonly id: string;
  title: string;
  description: string | undefined;
  created_at: Date;
}

export interface Content {
  seconds: number;
  kind: ContentKind;
  path: string;
}

export type ContentKind = "Video" | "Audio";

// deno-lint-ignore no-empty-interface
export interface Folder {
  // now is empty
}

export type ItemKind = "Content" | "Folder";

export type ItemDetail = Item &
  ({ kind: "Content"; detail: Content } | { kind: "Folder"; detail: Folder });

export interface FolderNode {
  folder: string;
  items: ItemNode[];
}

export type ItemNode = FolderNode | string;
