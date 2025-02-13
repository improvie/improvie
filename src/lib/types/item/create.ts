export type CreateBaseItem = {
  parent_folder_id: string;
  sort_order: number;

  title: string;
  description: string | undefined;
};

export type CreateFolder = CreateBaseItem & {};

export type CreateContent = CreateBaseItem & {
  kind: "Video" | "Audio";
  content_path: string;
  thumbnail_path: string | undefined;
};
