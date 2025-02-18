export type FileDialogResponse =
  | undefined
  | {
    path: string;
    name: string;
  };

export type ContentFileDialogResponse = FileDialogResponse & { kind: 'Audio' | 'Video' };
export type ImageFileDialogResponse = FileDialogResponse & { kind: 'Image' };
