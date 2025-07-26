ALTER TABLE recent_contents
  ADD COLUMN total_accesses int unsigned NOT NULL;

ALTER TABLE recent_playlists
  ADD COLUMN total_accesses int unsigned NOT NULL;
