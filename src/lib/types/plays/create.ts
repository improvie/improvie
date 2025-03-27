import type { PlayFolder, PlayFolderNode, Playlist } from '.';

export interface CreateBasePlayItem {
  parent_folder_id: string;

  title: string;
  description: string | undefined;
}

export type CreatePlayFolder = CreateBasePlayItem & {};

export type CreatePlaylist = CreateBasePlayItem & {
  thumbnail_path: string | undefined;
};

export interface CreatePlaylistResponse {
  playlist: Playlist;
  folder_node: PlayFolderNode;
}

export interface CreatePlayFolderResponse {
  folder: PlayFolder;
  folder_node: PlayFolderNode;
}
