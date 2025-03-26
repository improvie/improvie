CREATE TABLE IF NOT EXISTS play_items (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    kind tinyint unsigned NOT NULL,
    created_at timestamp NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS play_folders (
    item_id uuid NOT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES play_items (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS playlists (
    item_id uuid NOT NULL,

    thumbnail_path text DEFAULT NULL,
    rules json NOT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES play_items (id) ON DELETE CASCADE
);

INSERT INTO play_items (id, title, kind, created_at) VALUES (
    '00000000-0000-0000-0000-000000000000', 'Root', 1, (DATETIME('now'))
);

INSERT INTO play_folders (item_id) VALUES (
    '00000000-0000-0000-0000-000000000000'
);

CREATE TABLE IF NOT EXISTS hierarchical_play_items (
    parent_folder_id uuid NOT NULL,
    child_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    created_at timestamp NOT NULL,

    PRIMARY KEY (parent_folder_id, child_id),
    FOREIGN KEY (parent_folder_id) REFERENCES play_folders (item_id) ON DELETE CASCADE,
    FOREIGN KEY (child_id) REFERENCES play_items (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS favorite_playlists (
    playlist_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    PRIMARY KEY (playlist_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
);
