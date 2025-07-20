use sea_orm::{ColumnTrait, FromQueryResult, Statement, TryGetableMany};
use std::collections::HashMap;

use chrono::Utc;
use improvie_domain::{
    model::plays::{CreatePlayFolderModel, CreatePlaylistModel},
    repository::plays::PlaystsRepository,
};
use improvie_logic::{
    DynAppResult,
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayFolderNode, PlayItem, PlayItemNode, Playlist},
};
use more_convert::VecInto;
use sea_orm::{EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uid::Uid;

use crate::{
    model::plays::{PlayCurrentNodeRaw, PlayFolderRow, PlayNodeRaw, PlaylistRow},
    repository::{insert_check, modify_check},
};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaystsRepository for PlaylistsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnectionImpl<'a>;
    async fn get_play_folders(&self) -> DynAppResult<Vec<PlayFolder>> {
        let rows = improvie_row::play_folders::Entity::find()
            .select_only()
            .inner_join(improvie_row::play_items::Entity)
            .column(improvie_row::play_items::Column::Id)
            .column(improvie_row::play_items::Column::Title)
            .column(improvie_row::play_items::Column::Description)
            .column(improvie_row::play_items::Column::CreatedAt)
            .into_model::<PlayFolderRow>()
            .all(self.db.pool())
            .await?;

        Ok(rows.vec_into())
    }

    async fn get_playlists(&self) -> DynAppResult<Vec<Playlist>> {
        let rows = improvie_row::playlists::Entity::find()
            .select_only()
            .column(improvie_row::playlists::Column::ThumbnailPath)
            .inner_join(improvie_row::play_items::Entity)
            .column(improvie_row::play_items::Column::Id)
            .column(improvie_row::play_items::Column::Title)
            .column(improvie_row::play_items::Column::Description)
            .column(improvie_row::play_items::Column::CreatedAt)
            .into_model::<PlaylistRow>()
            .all(self.db.pool())
            .await?;

        Ok(rows.vec_into())
    }

    async fn get_favorite_playlists(&self) -> DynAppResult<Vec<Uid>> {
        let rows = improvie_row::favorite_playlists::Entity::find()
            .select_only()
            .column(improvie_row::favorite_playlists::Column::PlaylistId)
            .order_by(
                improvie_row::favorite_playlists::Column::SortOrder,
                sea_orm::Order::Asc,
            )
            .into_tuple::<Uid>()
            .all(self.db.pool())
            .await?;

        Ok(rows.vec_into())
    }

    async fn add_favorite_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: Uid,
    ) -> DynAppResult<()> {
        let max_number: u32 = improvie_row::favorite_playlists::Entity::find()
            .select_only()
            .column(improvie_row::favorite_playlists::Column::SortOrder)
            .into_tuple::<u32>()
            .one(&conn)
            .await?
            .unwrap_or(0);

        let result = improvie_row::favorite_playlists::Entity::insert(
            improvie_row::favorite_playlists::ActiveModel {
                playlist_id: sea_orm::Set(playlist_id),
                sort_order: sea_orm::Set(max_number + 1),
            },
        )
        .exec_without_returning(&conn)
        .await;

        insert_check!(result);

        Ok(())
    }

    async fn remove_favorite_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: Uid,
    ) -> DynAppResult<()> {
        let result = improvie_row::favorite_playlists::Entity::delete_many()
            .filter(improvie_row::favorite_playlists::Column::PlaylistId.eq(playlist_id))
            .exec(&conn)
            .await;

        modify_check!(result);

        Ok(())
    }

    async fn get_plays_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<PlayFolderNode> {
        let rows = improvie_row::hierarchical_play_items::Entity::find()
            .select_only()
            .column(improvie_row::hierarchical_play_items::Column::ChildId)
            .column(improvie_row::hierarchical_play_items::Column::SortOrder)
            .inner_join(improvie_row::play_items::Entity)
            .column_as(improvie_row::play_items::Column::Kind, "child_kind")
            .filter(improvie_row::hierarchical_play_items::Column::ParentFolderId.eq(folder_id))
            .into_model::<PlayCurrentNodeRaw>()
            .all(self.db.pool())
            .await?;

        let mut children: Vec<PlayItemNode> = vec![];
        for row in rows {
            match row.child_kind {
                PlayItemKind::Folder => {
                    children.push(PlayItemNode::Folder {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
                PlayItemKind::Playlist => {
                    children.push(PlayItemNode::Playlist {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
            }
        }

        Ok(PlayFolderNode {
            folder: folder_id,
            children,
        })
    }

    async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, PlayFolderNode>> {
        let rows = PlayNodeRaw::find_by_statement(Statement::from_sql_and_values(
            self.db.backend(),
            "
WITH RECURSIVE folder_hierarchy(parent_folder_id, child_id, child_kind, sort_order) AS (
    SELECT
        hi.parent_folder_id,
        hi.child_id,
        i.kind AS child_kind,
        hi.sort_order
    FROM hierarchical_play_items AS hi
    INNER JOIN play_items AS i ON i.id = hi.child_id
    WHERE hi.parent_folder_id = ?

    UNION ALL

    SELECT
        hi.parent_folder_id,
        hi.child_id,
        i.kind AS child_kind,
        hi.sort_order
    FROM hierarchical_play_items AS hi
    INNER JOIN folder_hierarchy AS fh ON hi.parent_folder_id = fh.child_id
    INNER JOIN play_items AS i ON hi.child_id = i.id
)
SELECT *
FROM folder_hierarchy
",
            [folder_id.into()],
        ))
        .all(self.db.pool())
        .await?;

        let mut nodes: HashMap<Uid, PlayFolderNode> = HashMap::new();
        for row in rows {
            let parent = nodes
                .entry(row.parent_folder_id)
                .or_insert_with(|| PlayFolderNode {
                    folder: row.parent_folder_id,
                    children: vec![],
                });
            match row.child_kind {
                PlayItemKind::Folder => {
                    parent.children.push(PlayItemNode::Folder {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
                PlayItemKind::Playlist => {
                    parent.children.push(PlayItemNode::Playlist {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
            }
        }
        Ok(nodes)
    }

    async fn create_play_folder(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreatePlayFolderModel,
    ) -> DynAppResult<PlayFolder> {
        let folder = PlayFolder {
            item: PlayItem {
                id: Uid::new(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
        };

        add_play_item(conn, &folder.item, PlayItemKind::Folder).await?;

        let folder_result =
            improvie_row::play_folders::Entity::insert(improvie_row::play_folders::ActiveModel {
                item_id: sea_orm::Set(folder.item.id),
            })
            .exec_without_returning(&conn)
            .await;

        insert_check!(folder_result);

        add_play_hierarchy(conn, model.item.parent_folder_id, folder.item.id).await?;

        Ok(folder)
    }

    async fn create_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreatePlaylistModel,
    ) -> DynAppResult<Playlist> {
        let content = Playlist {
            item: PlayItem {
                id: Uid::new(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
            thumbnail_path: model.thumbnail_path,
        };

        add_play_item(conn, &content.item, PlayItemKind::Playlist).await?;

        let playlist_result =
            improvie_row::playlists::Entity::insert(improvie_row::playlists::ActiveModel {
                item_id: sea_orm::Set(content.item.id),
                thumbnail_path: sea_orm::Set(content.thumbnail_path.clone()),
                rules: sea_orm::Set(String::new()),
            })
            .exec_without_returning(&conn)
            .await;

        insert_check!(playlist_result);

        add_play_hierarchy(conn, model.item.parent_folder_id, content.item.id).await?;

        Ok(content)
    }

    async fn delete_play_item(
        &self,
        conn: Self::DbConnection<'_>,
        play_id: Uid,
    ) -> DynAppResult<Vec<Uid>> {
        let mut play_item_uids = Uid::find_by_statement::<improvie_row::play_items::Column>(
            Statement::from_sql_and_values(
                self.db.backend(),
                "
WITH RECURSIVE item_hierarchy(child_id) AS (
    SELECT
        hi.child_id
    FROM hierarchical_play_items AS hi
    WHERE hi.parent_folder_id = ?

    UNION ALL

    SELECT
        hi.child_id
    FROM hierarchical_play_items AS hi
    INNER JOIN item_hierarchy AS ih ON hi.parent_folder_id = ih.child_id
)
SELECT child_id
FROM item_hierarchy
",
                [play_id.into()],
            ),
        )
        .all(&conn)
        .await?;

        play_item_uids.push(play_id);

        let result = improvie_row::play_items::Entity::delete_many()
            .filter(improvie_row::play_items::Column::Id.is_in(play_item_uids.clone()))
            .exec(&conn)
            .await;

        modify_check!(result);

        Ok(play_item_uids)
    }

    async fn update_play_item_name(
        &self,
        conn: Self::DbConnection<'_>,
        play_id: Uid,
        name: String,
    ) -> DynAppResult<()> {
        let result = improvie_row::play_items::Entity::update_many()
            .set(improvie_row::play_items::ActiveModel {
                title: sea_orm::Set(name.clone()),
                ..Default::default()
            })
            .filter(improvie_row::play_items::Column::Id.eq(play_id))
            .exec(&conn)
            .await;

        modify_check!(result);

        Ok(())
    }
}

async fn add_play_item(
    conn: crate::persistence::db::DbConnectionImpl<'_>,
    item: &PlayItem,
    kind: PlayItemKind,
) -> DynAppResult<()> {
    let item_result =
        improvie_row::play_items::Entity::insert(improvie_row::play_items::ActiveModel {
            id: sea_orm::Set(item.id),
            title: sea_orm::Set(item.title.clone()),
            description: sea_orm::Set(item.description.clone()),
            kind: sea_orm::Set(kind),
            created_at: sea_orm::Set(item.created_at),
        })
        .exec_without_returning(&conn)
        .await;

    insert_check!(item_result);

    Ok(())
}

async fn add_play_hierarchy(
    conn: crate::persistence::db::DbConnectionImpl<'_>,
    parent_folder_id: Uid,
    item_id: Uid,
) -> DynAppResult<()> {
    use improvie_row::hierarchical_play_items;

    let sort_order = hierarchical_play_items::Entity::find()
        .select_only()
        .column(hierarchical_play_items::Column::SortOrder)
        .filter(hierarchical_play_items::Column::ParentFolderId.eq(parent_folder_id))
        .order_by_asc(hierarchical_play_items::Column::SortOrder)
        .into_tuple::<u32>()
        .one(&conn)
        .await?
        .unwrap_or(0);

    let sort_order = sort_order + 1;

    let shift_result = hierarchical_play_items::Entity::update_many()
        .filter(hierarchical_play_items::Column::ParentFolderId.eq(parent_folder_id))
        .filter(hierarchical_play_items::Column::SortOrder.gte(sort_order))
        .col_expr(
            hierarchical_play_items::Column::SortOrder,
            hierarchical_play_items::Column::SortOrder
                .into_expr()
                .add(1),
        )
        .exec(&conn)
        .await;

    // If affecting rows is 0, it means there are no items to shift. not an error.
    shift_result?;

    let hierarchy_result = improvie_row::hierarchical_play_items::Entity::insert(
        improvie_row::hierarchical_play_items::ActiveModel {
            parent_folder_id: sea_orm::Set(parent_folder_id),
            child_id: sea_orm::Set(item_id),
            sort_order: sea_orm::Set(sort_order),
            created_at: sea_orm::Set(Utc::now()),
        },
    )
    .exec_without_returning(&conn)
    .await;

    insert_check!(hierarchy_result);

    Ok(())
}
