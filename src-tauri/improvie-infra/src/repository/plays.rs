use std::collections::HashMap;

use chrono::Utc;
use improvie_domain::{
    model::plays::{CreatePlayFolderModel, CreatePlaylistModel},
    repository::plays::PlaystsRepository,
};
use improvie_logic::{
    AppResult,
    constant::plays::PlayItemKind,
    logic::rule::Rule,
    model::plays::{PlayFolder, PlayFolderNode, PlayItem, PlayItemNode, Playlist},
};
use more_convert::VecInto;
use uuid::Uuid;

use crate::{
    model::plays::{PlayCurrentNodeRaw, PlayFolderRow, PlayNodeRaw, PlaylistRow},
    persistence::db::DbTx,
    repository::tx_check,
};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaystsRepository for PlaylistsRepositoryImpl {
    async fn get_play_folders(&self) -> AppResult<Vec<PlayFolder>> {
        let rows = sqlx::query_as::<_, PlayFolderRow>(
            "
SELECT 
    pi.id, pi.title, pi.description, pi.created_at
FROM play_folders AS pf
INNER JOIN play_items AS pi ON pi.id = pf.item_id
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        Ok(rows.vec_into())
    }

    async fn get_playlists(&self) -> AppResult<Vec<Playlist>> {
        let rows = sqlx::query_as::<_, PlaylistRow>(
            "
SELECT
    pi.id, pi.title, pi.description, pl.thumbnail_path, pi.created_at
FROM playlists AS pl
INNER JOIN play_items AS pi ON pi.id = pl.item_id
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

    async fn get_plays_hierarchy_current(&self, folder_id: Uuid) -> AppResult<PlayFolderNode> {
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
        folder_id: Uuid,
    ) -> AppResult<HashMap<Uuid, PlayFolderNode>> {
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

        let mut nodes: HashMap<Uuid, PlayFolderNode> = HashMap::new();
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

    async fn create_play_folder(&self, model: CreatePlayFolderModel) -> AppResult<PlayFolder> {
        let folder = PlayFolder {
            item: PlayItem {
                id: Uuid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
        };

        let mut tx = self.db.begin().await?;

        add_play_item(&mut tx, &folder.item, PlayItemKind::Folder).await?;

        let folder_result = sqlx::query("INSERT INTO play_folders (item_id) VALUES (?)")
            .bind(folder.item.id)
            .execute(&mut *tx)
            .await;

        tx_check!(folder_result, tx);

        add_play_hierarchy(&mut tx, model.item.parent_folder_id, folder.item.id).await?;

        tx.commit().await?;

        Ok(folder)
    }

    async fn create_playlist(&self, model: CreatePlaylistModel) -> AppResult<Playlist> {
        let content = Playlist {
            item: PlayItem {
                id: Uuid::now(),
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
                .bind(sqlx::types::Json(Vec::<Rule>::new()))
                .execute(&mut *tx)
                .await;

        tx_check!(playlist_result, tx);

        add_play_hierarchy(&mut tx, model.item.parent_folder_id, content.item.id).await?;

        tx.commit().await?;

        Ok(content)
    }
}

async fn add_play_item(tx: &mut DbTx, item: &PlayItem, kind: PlayItemKind) -> AppResult<()> {
    let item_result = sqlx::query(
        "INSERT INTO play_items (id, title, description, kind, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(item.id)
    .bind(&item.title)
    .bind(&item.description)
    .bind(kind)
    .bind(item.created_at)
    .execute(&mut **tx)
    .await;

    tx_check!(item_result, tx);

    Ok(())
}

async fn add_play_hierarchy(tx: &mut DbTx, parent_folder_id: Uuid, item_id: Uuid) -> AppResult<()> {
    let sort_order: u32 = sqlx::query_scalar(
        "
SELECT 
    MAX(sort_order)
FROM hierarchical_play_items
WHERE parent_folder_id = ?
",
    )
    .bind(parent_folder_id)
    .fetch_one(&mut **tx)
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
    .execute(&mut **tx)
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
    .execute(&mut **tx)
    .await;

    tx_check!(hierarchy_result, tx);

    Ok(())
}
