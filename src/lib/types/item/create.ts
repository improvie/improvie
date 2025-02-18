import type { ContentKind } from '.';

export interface CreateBaseItem {
  parent_folder_id: string;
  sort_order: number;

  title: string;
  description: string | undefined;
}

export type CreateFolder = CreateBaseItem & {};

export type CreateContent = CreateBaseItem & {
  kind: ContentKind;
  content_path: string;
  thumbnail_path: string | undefined;
};
