CREATE TABLE IF NOT EXISTS settings (
    id uuid NOT NULL,
    details json DEFAULT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);
