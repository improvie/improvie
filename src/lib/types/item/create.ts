import type { Content, ContentKind, Folder, FolderNode } from '.';

export interface CreateBaseItem {
  parent_folder_id: string;

  title: string;
  description: string | undefined;
}

export type CreateFolder = CreateBaseItem & {};

export type CreateContent = CreateBaseItem & {
  kind: ContentKind;
  content_path: string;
  thumbnail_path: string | undefined;
};

export interface CreateContentResponse {
  content: Content;
  folder_node: FolderNode;
}

export interface CreateFolderResponse {
  folder: Folder;
  folder_node: FolderNode;
}
