CREATE TABLE IF NOT EXISTS items (
    id uuid NOT NULL,
    title text NOT NULL,
    description text DEFAULT NULL,
    kind tinyint unsigned NOT NULL,
    created_at timestamp NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS folders (
    item_id uuid NOT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS contents (
    item_id uuid NOT NULL,
    kind tinyint unsigned NOT NULL,
    content_path text NOT NULL,
    thumbnail_path text DEFAULT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE
);

INSERT INTO items (id, title, kind, created_at) VALUES (
    '00000000-0000-0000-0000-000000000000', 'Root', 1, (DATETIME('now'))
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
