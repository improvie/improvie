import type { ContentKind } from './item';

export type FileDialogResponse =
  | undefined
  | {
    path: string;
    name: string;
  };

export type ContentFileDialogResponse = FileDialogResponse & { kind: ContentKind };
export type ImageFileDialogResponse = FileDialogResponse & { kind: 'Image' };
