use std::collections::HashMap;

use improvie_domain::repository::playlists::PlaylistsRepository;
use improvie_logic::{
    model::playlist::{Playlist, PlaylistFolder},
    AppResult, Uuid,
};
use more_convert::VecInto;

use crate::model::playlists::{PlaylistFolderRow, PlaylistRow};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaylistsRepository for PlaylistsRepositoryImpl {
    async fn get_playlist_folders(&self) -> AppResult<HashMap<Uuid, Vec<PlaylistFolder>>> {
        let rows = sqlx::query_as::<_, PlaylistFolderRow>(
            "
SELECT 
    id, title, description, parent_id, sort_order
FROM plays
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        let mut group: HashMap<Uuid, Vec<PlaylistFolder>> = Default::default();
        for row in rows {
            group.entry(row.parent_id).or_default().push(row.into());
        }

        Ok(group)
    }

    async fn get_playlists(&self) -> AppResult<HashMap<Uuid, Vec<Playlist>>> {
        let rows = sqlx::query_as::<_, PlaylistRow>(
            "
SELECT 
    id, title, description, thumbnail_path, rules, folder_id, sort_order
FROM playlists
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        let mut group: HashMap<Uuid, Vec<Playlist>> = Default::default();
        for row in rows {
            group.entry(row.folder_id).or_default().push(row.into());
        }

        Ok(group)
    }

    async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>> {
        let rows = sqlx::query_scalar::<_, Uuid>(
            "
SELECT playlist_id
FROM favorite_playlists
ORDER BY sort_order
",
        )
        .bind(Uuid::nil())
        .fetch_all(&self.db.pool())
        .await?;

        Ok(rows.vec_into())
    }
}
