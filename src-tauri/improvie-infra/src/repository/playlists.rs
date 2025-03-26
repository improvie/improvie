use std::collections::HashMap;

use improvie_domain::repository::playlists::PlaystsRepository;
use improvie_logic::{
    AppResult,
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayFolderNode, PlayItemNode, Playlist},
};
use more_convert::VecInto;
use uuid::Uuid;

use crate::model::playlists::{PlayCurrentNodeRaw, PlayFolderRow, PlayNodeRaw, PlaylistRow};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaystsRepository for PlaylistsRepositoryImpl {
    async fn get_play_folders(&self) -> AppResult<Vec<PlayFolder>> {
        let rows = sqlx::query_as::<_, PlayFolderRow>(
            "
SELECT 
    pi.id, pi.title, pi.description
FROM play_folders AS pf
INNER JOIN play_items AS pi ON pi.id = pf.id
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
    pi.id, pi.title, pi.description, pl.thumbnail_path, pl.rules
FROM playlists AS pl
INNER JOIN play_items AS pi ON pi.id = pl.id
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
    INNER JOIN items AS i ON i.id = hi.child_id
    WHERE hi.parent_folder_id = ?

    UNION ALL

    SELECT
        hi.parent_folder_id,
        hi.child_id,
        i.kind AS child_kind,
        hi.sort_order
    FROM hierarchical_play_items AS hi
    INNER JOIN folder_hierarchy AS fh ON hi.parent_folder_id = fh.child_id
    INNER JOIN items AS i ON hi.child_id = i.id
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
}
