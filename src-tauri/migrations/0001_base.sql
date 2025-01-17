CREATE TABLE IF NOT EXISTS settings (
    id uuid NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS folders (
    id uuid NOT NULL,
    title varchar(255) NOT NULL,
    description text DEFAULT NULL,
    visibility tinyint unsigned NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS hierarchical_folders (
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (parent_id, child_id),
    FOREIGN KEY (parent_id) REFERENCES folders (id) ON DELETE CASCADE,
    FOREIGN KEY (child_id) REFERENCES folders (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contents (
    id uuid NOT NULL,
    title varchar(255) NOT NULL,
    description text DEFAULT NULL,
    seconds int unsigned NOT NULL,
    kind tinyint unsigned NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS folder_contents (
    folder_id uuid NOT NULL,
    content_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (folder_id, content_id),
    FOREIGN KEY (folder_id) REFERENCES folders (id) ON DELETE CASCADE,
    FOREIGN KEY (content_id) REFERENCES contents (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS playlists (
    id uuid NOT NULL,
    title varchar(255) NOT NULL,
    description text DEFAULT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS plays (
    id uuid NOT NULL,
    title varchar(255) NOT NULL,
    description text DEFAULT NULL,
    rules json NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS playlist_plays (
    playlist_id uuid NOT NULL,
    play_id uuid NOT NULL,
    sort_order int unsigned NOT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (playlist_id, play_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE,
    FOREIGN KEY (play_id) REFERENCES plays (id) ON DELETE CASCADE
);
