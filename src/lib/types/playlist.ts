export interface Playlist {
  readonly id: string;
  title: string;
  description: string | undefined;
  thumbnail_path: string | undefined;
  sort_order: number;
}

export interface Play {
  readonly id: string;
  title: string;
  description: string | undefined;
  sort_order: number;
}
