export interface Item {
  readonly id: string;
  title: string;
  description: string | undefined;
  created_at: Date;
}

export interface Content extends Item {
  kind: ContentKind;
  content_path: string;
  thumbnail_path: string | undefined;
}

export type ContentKind = 'Video' | 'Audio';

export interface Folder extends Item {
  // currency empty
}

export type ItemKind = 'Content' | 'Folder';

export interface FolderNode {
  folder: string;
  items: ItemNode[];
}

export interface ItemNode {
  kind: ItemKind;
  id: string;
  sort_order: number;
};

export interface PickItem {
  id: string;
  hierarchy_name: string;
}
