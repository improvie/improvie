CREATE TABLE IF NOT EXISTS playlist_folders (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    details json DEFAULT NULL,

    parent_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,

    created_at timestamp NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (parent_id) REFERENCES playlists (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS playlists (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    thumbnail_path text DEFAULT NULL,
    details json DEFAULT NULL,

    rules json NOT NULL,

    folder_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,

    created_at timestamp NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (folder_id) REFERENCES playlist_folder
);

CREATE TABLE IF NOT EXISTS favorite_playlists (
    playlist_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    PRIMARY KEY (playlist_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
);
