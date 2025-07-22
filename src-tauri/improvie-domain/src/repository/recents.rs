use improvie_logic::DynAppResult;

#[async_trait::async_trait]
pub trait RecentsRepository {
    type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

    async fn update_content(
        &self,
        conn: Self::DbConnection<'_>,
        content_id: uid::Uid,
    ) -> DynAppResult<()>;

    async fn update_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: uid::Uid,
    ) -> DynAppResult<()>;

    async fn get_recent_contents(
        &self,
        conn: Self::DbConnection<'_>,
        limit: Option<u64>,
    ) -> DynAppResult<Vec<uid::Uid>>;

    async fn get_recent_playlists(
        &self,
        conn: Self::DbConnection<'_>,
        limit: Option<u64>,
    ) -> DynAppResult<Vec<uid::Uid>>;
}
