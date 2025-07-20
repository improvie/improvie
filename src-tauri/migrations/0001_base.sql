CREATE TABLE IF NOT EXISTS app_settings (
    id varchar(26) NOT NULL,
    settings json NOT NULL,
    created_at timestamp NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO app_settings (id, settings, created_at) VALUES
('00000000000000000000000000', '{}', (DATETIME('now')));
