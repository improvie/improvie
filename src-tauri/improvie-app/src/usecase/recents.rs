use improvie_domain::{
    modules::RepositoriesModule, persistence::db::DbTx, repository::recents::RecentsRepository,
};
use improvie_logic::model::utils::RangeLimit;

super::def_use_case!(RecentsUseCase);

impl<R: RepositoriesModule> RecentsUseCase<R> {
    pub async fn update_content(&self, content_id: uid::Uid) -> improvie_logic::DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .recents_repository()
            .update_content(conn, content_id)
            .await;

        super::tx_commit!(tx, result)
    }

    pub async fn update_playlist(&self, playlist_id: uid::Uid) -> improvie_logic::DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .recents_repository()
            .update_playlist(conn, playlist_id)
            .await;

        super::tx_commit!(tx, result)
    }

    pub async fn get_recent_contents(
        &self,
        limit: Option<u64>,
        duration_range: RangeLimit,
    ) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
        self.repository
            .recents_repository()
            .get_recent_contents(self.repository.connection(), limit, duration_range)
            .await
    }

    pub async fn get_recent_playlists(
        &self,
        limit: Option<u64>,
    ) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
        self.repository
            .recents_repository()
            .get_recent_playlists(self.repository.connection(), limit)
            .await
    }
}
