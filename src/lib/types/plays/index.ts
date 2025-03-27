export interface PlayItem {
  readonly id: string;
  title: string;
  description: string | undefined;
  created_at: Date;
}

export interface Playlist extends PlayItem {
  thumbnail_path: string | undefined;
  // TODO: rules
  rules: string;
}

export interface PlayFolder extends PlayItem {
  // currency empty
}

export type PlayItemKind = 'Playlist' | 'Folder';

export type PlayItemDetail = PlayItem &
  ({ kind: 'Playlist'; detail: Playlist } | { kind: 'Folder'; detail: PlayFolder });

export interface PlayFolderNode {
  folder: string;
  children: PlayItemNode[];
}

export interface PlayItemNode {
  kind: PlayItemKind;
  id: string;
  sort_order: number;
};
