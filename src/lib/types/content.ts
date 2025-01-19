export interface Content {
  readonly id: string;
  title: string;
  description: string | undefined;
  seconds: number;
  kind: ContentKind;
  path: string;
  created_at: Date;
}

export type ContentKind = "Video" | "Audio";

export interface Folder {
  readonly id: string;
  title: string;
  description: string | undefined;
  created_at: Date;
}

export type ItemNode = Content | FolderNode;

export interface FolderNode {
  folder: Folder;
  children: ItemNode[];
}
