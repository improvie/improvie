use improvie_domain::repository::recents::RecentsRepository;
use sea_orm::{EntityTrait, QueryOrder, QuerySelect, sea_query::OnConflict};

super::def_repository_impl!(RecentsRepositoryImpl);

#[async_trait::async_trait]
impl RecentsRepository for RecentsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnectionImpl<'a>;

    async fn update_content(
        &self,
        conn: Self::DbConnection<'_>,
        content_id: uid::Uid,
    ) -> improvie_logic::DynAppResult<()> {
        use improvie_row::recent_contents::*;

        let model = ActiveModel {
            content_id: sea_orm::Set(content_id),
            last_accessed: sea_orm::Set(chrono::Utc::now()),
        };

        let result = Entity::insert(model)
            .on_conflict(
                OnConflict::column(Column::ContentId)
                    .update_column(Column::LastAccessed)
                    .to_owned(),
            )
            .exec_without_returning(&conn)
            .await;

        super::insert_check!(result);

        Ok(())
    }

    async fn update_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: uid::Uid,
    ) -> improvie_logic::DynAppResult<()> {
        use improvie_row::recent_playlists::*;

        let model = ActiveModel {
            playlist_id: sea_orm::Set(playlist_id),
            last_accessed: sea_orm::Set(chrono::Utc::now()),
        };

        let result = Entity::insert(model)
            .on_conflict(
                OnConflict::column(Column::PlaylistId)
                    .update_column(Column::LastAccessed)
                    .to_owned(),
            )
            .exec_without_returning(&conn)
            .await;

        super::insert_check!(result);

        Ok(())
    }

    async fn get_recent_contents(
        &self,
        conn: Self::DbConnection<'_>,
        limit: Option<u64>,
    ) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
        use improvie_row::recent_contents::*;

        let mut query = Entity::find()
            .select_only()
            .column(Column::ContentId)
            .order_by_desc(Column::LastAccessed);

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        query
            .into_tuple::<uid::Uid>()
            .all(&conn)
            .await
            .map_err(Into::into)
    }

    async fn get_recent_playlists(
        &self,
        conn: Self::DbConnection<'_>,
        limit: Option<u64>,
    ) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
        use improvie_row::recent_playlists::*;

        let mut query = Entity::find()
            .select_only()
            .column(Column::PlaylistId)
            .order_by_desc(Column::LastAccessed);

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        query
            .into_tuple::<uid::Uid>()
            .all(&conn)
            .await
            .map_err(Into::into)
    }
}
