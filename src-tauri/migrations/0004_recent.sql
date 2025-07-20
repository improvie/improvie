CREATE TABLE IF NOT EXISTS recent_contents (
    item_id varchar(26) NOT NULL,
    last_accessed timestamp NOT NULL,

    PRIMARY KEY (item_id),
    FOREIGN KEY (item_id) REFERENCES contents (item_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS recent_playlists (
    playlist_id varchar(26) NOT NULL,
    last_accessed timestamp NOT NULL,

    PRIMARY KEY (playlist_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (item_id) ON DELETE CASCADE
)
