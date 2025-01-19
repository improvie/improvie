export interface Playlist {
  readonly id: string;
  title: string;
  description: string | undefined;
  emoji: string | undefined;
}

export interface Play {
  readonly id: string;
  title: string;
  description: string | undefined;
}
