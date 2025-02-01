use improvie_domain::repository::playlists::PlaylistsRepository;
use improvie_logic::{
    model::playlist::{Play, Playlist},
    AppResult, Uuid,
};
use more_convert::VecInto;

use crate::model::playlists::{PlaylistsRaw, PlaysRaw};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaylistsRepository for PlaylistsRepositoryImpl {
    async fn get_playlists(&self) -> AppResult<Vec<Playlist>> {
        let rows = sqlx::query_as::<_, PlaylistsRaw>(
            "
SELECT 
    id, title, description, thumbnail_path, sort_order
FROM playlists
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        Ok(rows.vec_into())
    }

    async fn get_plays(&self) -> AppResult<Vec<Play>> {
        let rows = sqlx::query_as::<_, PlaysRaw>(
            "
SELECT 
    id, title, description, rules, sort_order
FROM plays
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        Ok(rows.vec_into())
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
