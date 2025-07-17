use std::collections::HashMap;

use chrono::Utc;
use improvie_domain::{
    model::{
        plays::{CreatePlayFolderModel, CreatePlaylistModel},
        rules::RuleData,
    },
    repository::plays::PlaystsRepository,
};
use improvie_logic::{
    DynAppResult,
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayFolderNode, PlayItem, PlayItemNode, Playlist},
};
use more_convert::VecInto;
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
use sqlx::QueryBuilder;
use uid::Uid;

use crate::{
    model::plays::{PlayCurrentNodeRaw, PlayFolderRow, PlayNodeRaw, PlaylistRow},
    persistence::db::DbTx,
    repository::tx_check,
};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaystsRepository for PlaylistsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnection<'a>;
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

    async fn add_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()> {
        let mut tx = self.db.begin().await?;

        let max_number: u32 = sqlx::query_scalar(
            "
SELECT
    MAX(sort_order)
FROM favorite_playlists
",
        )
        .fetch_one(tx.as_mut())
        .await?;

        let result = sqlx::query(
            "
INSERT INTO favorite_playlists (playlist_id, sort_order) VALUES (?, ?)",
        )
        .bind(playlist_id)
        .bind(max_number + 1)
        .execute(tx.as_mut())
        .await;

        tx_check!(result, tx);

        tx.commit().await?;

        Ok(())
    }

    async fn remove_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()> {
        let mut tx = self.db.begin().await?;

        let result = sqlx::query(
            "
DELETE FROM favorite_playlists
WHERE playlist_id = ?",
        )
        .bind(playlist_id)
        .execute(tx.as_mut())
        .await;

        tx_check!(result, tx);

        tx.commit().await?;

        Ok(())
    }

    async fn get_plays_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<PlayFolderNode> {
        let rows = sqlx::query_as::<_, PlayCurrentNodeRaw>(
            "
SELECT
    hpi.child_id, pi.kind AS child_kind, hpi.sort_order
FROM hierarchical_play_items AS hpi
INNER JOIN play_items AS pi ON pi.id = hpi.child_id
WHERE hpi.parent_folder_id = ?
",
        )
        .bind(folder_id)
        .fetch_all(&self.db.pool())
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
        let rows = sqlx::query_as::<_, PlayNodeRaw>(
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
        )
        .bind(folder_id)
        .fetch_all(&self.db.pool())
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

    async fn create_play_folder(&self, model: CreatePlayFolderModel) -> DynAppResult<PlayFolder> {
        let folder = PlayFolder {
            item: PlayItem {
                id: Uid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
        };

        let mut tx = self.db.begin().await?;

        add_play_item(&mut tx, &folder.item, PlayItemKind::Folder).await?;

        let folder_result = sqlx::query("INSERT INTO play_folders (item_id) VALUES (?)")
            .bind(folder.item.id)
            .execute(tx.as_mut())
            .await;

        tx_check!(folder_result, tx);

        add_play_hierarchy(&mut tx, model.item.parent_folder_id, folder.item.id).await?;

        tx.commit().await?;

        Ok(folder)
    }

    async fn create_playlist(&self, model: CreatePlaylistModel) -> DynAppResult<Playlist> {
        let content = Playlist {
            item: PlayItem {
                id: Uid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
            thumbnail_path: model.thumbnail_path,
        };

        let mut tx = self.db.begin().await?;

        add_play_item(&mut tx, &content.item, PlayItemKind::Playlist).await?;

        let playlist_result =
            sqlx::query("INSERT INTO playlists (item_id, thumbnail_path, rules) VALUES (?, ?, ?)")
                .bind(content.item.id)
                .bind(&content.thumbnail_path)
                .bind(sqlx::types::Json(Vec::<RuleData>::new()))
                .execute(tx.as_mut())
                .await;

        tx_check!(playlist_result, tx);

        add_play_hierarchy(&mut tx, model.item.parent_folder_id, content.item.id).await?;

        tx.commit().await?;

        Ok(content)
    }

    async fn delete_play_item(&self, play_id: Uid) -> DynAppResult<Vec<Uid>> {
        let mut tx = self.db.begin().await?;

        let mut play_item_uids = sqlx::query_scalar::<_, Uid>(
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
        )
        .bind(play_id)
        .fetch_all(tx.as_mut())
        .await?;

        play_item_uids.push(play_id);

        let mut builder = QueryBuilder::new(
            "
DELETE FROM play_items
WHERE id IN (
",
        );
        let mut separated = builder.separated(", ");
        for id in &play_item_uids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");

        builder.build().execute(tx.as_mut()).await?;

        tx.commit().await?;

        Ok(play_item_uids)
    }

    async fn update_play_item_name(&self, play_id: Uid, name: String) -> DynAppResult<()> {
        let mut tx = self.db.begin().await?;
        let result = sqlx::query(
            "
UPDATE play_items
SET title = ?
WHERE id = ?
",
        )
        .bind(&name)
        .bind(play_id)
        .execute(tx.as_mut())
        .await;

        tx_check!(result, tx);

        tx.commit().await?;

        Ok(())
    }
}

async fn add_play_item(tx: &mut DbTx, item: &PlayItem, kind: PlayItemKind) -> DynAppResult<()> {
    let item_result = sqlx::query(
        "INSERT INTO play_items (id, title, description, kind, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(item.id)
    .bind(&item.title)
    .bind(&item.description)
    .bind(kind)
    .bind(item.created_at)
    .execute(tx.as_mut())
    .await;

    tx_check!(item_result, tx);

    Ok(())
}

async fn add_play_hierarchy(
    tx: &mut DbTx,
    parent_folder_id: Uid,
    item_id: Uid,
) -> DynAppResult<()> {
    let sort_order: u32 = sqlx::query_scalar(
        "
SELECT
    MAX(sort_order)
FROM hierarchical_play_items
WHERE parent_folder_id = ?
",
    )
    .bind(parent_folder_id)
    .fetch_one(tx.as_mut())
    .await?;

    let sort_order = sort_order + 1;

    let shift_result = sqlx::query(
        "
UPDATE hierarchical_play_items
SET sort_order = sort_order + 1
WHERE parent_folder_id = ? AND sort_order >= ?
",
    )
    .bind(parent_folder_id)
    .bind(sort_order)
    .execute(tx.as_mut())
    .await;

    shift_result?;

    let hierarchy_result = sqlx::query(
        "
INSERT INTO hierarchical_play_items
    (parent_folder_id, child_id, sort_order, created_at)
VALUES
    (?, ?, ?, ?)",
    )
    .bind(parent_folder_id)
    .bind(item_id)
    .bind(sort_order)
    .bind(Utc::now())
    .execute(tx.as_mut())
    .await;

    tx_check!(hierarchy_result, tx);

    Ok(())
}
