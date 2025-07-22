use improvie_domain::repository::recents::RecentsRepository;

super::def_repository_impl!(RecentsRepositoryImpl);

#[async_trait::async_trait]
impl RecentsRepository for RecentsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnectionImpl<'a>;

    async fn update_content(
        &self,
        conn: Self::DbConnection<'_>,
        content_id: uid::Uid,
    ) -> improvie_logic::DynAppResult<()> {
        todo!()
    }

    async fn update_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: uid::Uid,
    ) -> improvie_logic::DynAppResult<()> {
        todo!()
    }

    async fn get_recent_contents(
        &self,
        conn: Self::DbConnection<'_>,
        limit: Option<usize>,
    ) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
        todo!()
    }

    async fn get_recent_playlists(
        &self,
        conn: Self::DbConnection<'_>,
        limit: Option<usize>,
    ) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
        todo!()
    }
}
