export interface Item {
  readonly id: string;
  title: string;
  description: string | undefined;
  created_at: Date;
}

export interface Content extends Item {
  seconds: number;
  kind: ContentKind;
  content_path: string;
  thumbnail_path: string | undefined;
}

export type ContentKind = "Video" | "Audio";

export interface Folder extends Item {}

export type ItemKind = "Content" | "Folder";

export type ItemDetail = Item &
  ({ kind: "Content"; detail: Content } | { kind: "Folder"; detail: Folder });

export interface FolderNode {
  folder: string;
  items: ItemNode[];
}

export type ItemNode = ({ type: "Content" } | { type: "Folder" }) & {
  id: string;
  sort_order: number;
};

