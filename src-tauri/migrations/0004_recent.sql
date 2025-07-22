CREATE TABLE IF NOT EXISTS recent_contents (
    content_id varchar(26) NOT NULL,
    last_accessed timestamp NOT NULL,

    PRIMARY KEY (content_id),
    FOREIGN KEY (content_id) REFERENCES contents (item_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS recent_playlists (
    playlist_id varchar(26) NOT NULL,
    last_accessed timestamp NOT NULL,

    PRIMARY KEY (playlist_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists (item_id) ON DELETE CASCADE
)
