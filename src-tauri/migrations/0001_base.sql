CREATE TABLE IF NOT EXISTS settings (
    id uuid NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS items (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    details json DEFAULT NULL,
    kind tinyint unsigned NOT NULL,
    created_at timestamp NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS folders (
    item_id uuid NOT NULL,
    details json DEFAULT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contents (
    item_id uuid NOT NULL,
    seconds int unsigned NOT NULL,
    kind tinyint unsigned NOT NULL,
    content_path text NOT NULL,
    details json DEFAULT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
);

INSERT INTO items (id, title, kind, created_at) VALUES (
    '00000000-0000-0000-0000-000000000000', 'Root', 1, (DATETIME('now', 'localtime'))
);

INSERT INTO folders (item_id) VALUES (
    '00000000-0000-0000-0000-000000000000'
);

CREATE TABLE IF NOT EXISTS hierarchical_items (
    parent_folder_id uuid NOT NULL,
    child_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    created_at timestamp NOT NULL,

    PRIMARY KEY (parent_folder_id, child_id),
    FOREIGN KEY (parent_folder_id) REFERENCES folders (item_id) ON DELETE CASCADE,
    FOREIGN KEY (child_id) REFERENCES items (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS playlists (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS favorite_playlists (
    playlist_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    PRIMARY KEY (playlist_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS plays (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    rules json NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,

    playlist_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,

    PRIMARY KEY (id)
);
