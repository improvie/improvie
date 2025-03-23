CREATE TABLE IF NOT EXISTS playlists (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    thumbnail_path text DEFAULT NULL,

    rules json NOT NULL,

    folder_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,

    created_at timestamp NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (folder_id) REFERENCES playlist_folders (id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS playlist_folders (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,

    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS playlist_folder_hierarchy (
    parent_folder_id uuid NOT NULL,
    child_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    created_at timestamp NOT NULL,

    PRIMARY KEY (parent_folder_id, child_id),
    FOREIGN KEY (parent_folder_id) REFERENCES playlist_folders (id) ON DELETE CASCADE,
    FOREIGN KEY (child_id) REFERENCES playlist_folders (id) ON DELETE CASCADE
);

INSERT INTO playlist_folders (id, title, created_at) VALUES (
    '00000000-0000-0000-0000-000000000000', 'Root', (DATETIME('now'))
);


CREATE TABLE IF NOT EXISTS favorite_playlists (
    playlist_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    PRIMARY KEY (playlist_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
);
