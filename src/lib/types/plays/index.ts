export interface PlayItem {
  readonly id: string;
  title: string;
  description: string | undefined;
  created_at: Date;
}

export interface Playlist extends PlayItem {
  thumbnail_path: string | undefined;
}

export interface PlayFolder extends PlayItem {
  // currency empty
}

export type PlayItemKind = 'Playlist' | 'Folder';

export interface PlayFolderNode {
  folder: string;
  children: PlayItemNode[];
}

export interface PlayItemNode {
  kind: PlayItemKind;
  id: string;
  sort_order: number;
};
