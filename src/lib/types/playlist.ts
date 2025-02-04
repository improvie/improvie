export interface Playlist {
  readonly id: string;
  title: string;
  description: string | undefined;
  thumbnail_path: string | undefined;
  // TODO: rules
  rules: string;
  sort_order: number;
}

export interface PlaylistFolder {
  readonly id: string;
  title: string;
  description: string | undefined;
  sort_order: number;
}
