use std::collections::HashMap;

use chrono::Utc;
use improvie_domain::{
    model::items::{CreateContentModel, CreateFolderModel},
    repository::items::ItemsRepository,
};
use improvie_logic::{
    constant::items::ItemKind,
    model::items::{Content, Folder, FolderNode, Item, ItemNode},
    AppResult, Uuid,
};
use more_convert::VecInto;

use crate::model::items::{ContentRaw, CurrentNodeRaw, FolderRaw, NodeRaw};

use super::{def_repository_impl, tx_check};

def_repository_impl!(ItemsRepositoryImpl);

#[async_trait::async_trait]
impl ItemsRepository for ItemsRepositoryImpl {
    async fn get_items_hierarchy_current(&self, folder_id: Uuid) -> AppResult<FolderNode> {
        let rows = sqlx::query_as::<_, CurrentNodeRaw>(
            "
SELECT 
    hi.child_id, i.kind AS child_kind, hi.sort_order
FROM hierarchical_items AS hi
INNER JOIN items AS i ON i.id = hi.child_id
WHERE hi.parent_folder_id = ?
",
        )
        .bind(folder_id)
        .fetch_all(&self.db.pool())
        .await?;

        let mut items: Vec<ItemNode> = vec![];
        for row in rows {
            match row.child_kind {
                ItemKind::Folder => {
                    items.push(ItemNode::Folder {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
                ItemKind::Content => {
                    items.push(ItemNode::Content {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
            }
        }

        Ok(FolderNode {
            folder: folder_id,
            items,
        })
    }

    async fn get_items_hierarchy_loop(
        &self,
        folder_id: Uuid,
    ) -> AppResult<HashMap<Uuid, FolderNode>> {
        let rows = sqlx::query_as::<_, NodeRaw>(
            "
WITH RECURSIVE folder_hierarchy(parent_folder_id, child_id, child_kind, sort_order) AS (
    SELECT
        hi.parent_folder_id,
        hi.child_id,
        i.kind AS child_kind,
        hi.sort_order
    FROM hierarchical_items AS hi
    INNER JOIN items AS i ON i.id = hi.child_id
    WHERE hi.parent_folder_id = ?

    UNION ALL

    SELECT
        hi.parent_folder_id,
        hi.child_id,
        i.kind AS child_kind,
        hi.sort_order
    FROM hierarchical_items AS hi
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

        let mut nodes: HashMap<Uuid, FolderNode> = HashMap::new();
        for row in rows {
            let parent = nodes
                .entry(row.parent_folder_id)
                .or_insert_with(|| FolderNode {
                    folder: row.parent_folder_id,
                    items: vec![],
                });
            match row.child_kind {
                ItemKind::Folder => {
                    parent.items.push(ItemNode::Folder {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
                ItemKind::Content => {
                    parent.items.push(ItemNode::Content {
                        id: row.child_id,
                        sort_order: row.sort_order,
                    });
                }
            }
        }
        Ok(nodes)
    }

    async fn get_contents(&self) -> AppResult<Vec<Content>> {
        let row: Vec<ContentRaw> = sqlx::query_as(
            "
SELECT 
    i.id, i.title, i.description, i.created_at,
    c.kind, c.content_path, c.thumbnail_path
FROM contents AS c
INNER JOIN items AS i ON c.item_id = i.id
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        Ok(row.vec_into())
    }

    async fn get_folders(&self) -> AppResult<Vec<Folder>> {
        let row: Vec<FolderRaw> = sqlx::query_as(
            "
SELECT 
    i.id, i.title, i.description, i.created_at
FROM folders AS f
INNER JOIN items AS i ON f.item_id = i.id
",
        )
        .fetch_all(&self.db.pool())
        .await?;

        Ok(row.vec_into())
    }

    async fn create_folder(&self, model: CreateFolderModel) -> AppResult<Folder> {
        let folder = Folder {
            item: Item {
                id: Uuid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
        };

        let mut tx = self.db.begin().await?;

        let item_result = sqlx::query(
            "INSERT INTO items (id, title, description, kind, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(folder.item.id)
        .bind(&folder.item.title)
        .bind(&folder.item.description)
        .bind(ItemKind::Folder)
        .bind(folder.item.created_at)
        .execute(&mut *tx)
        .await;

        tx_check!(item_result, tx);

        let folder_result = sqlx::query("INSERT INTO folders (item_id) VALUES (?)")
            .bind(folder.item.id)
            .execute(&mut *tx)
            .await;

        tx_check!(folder_result, tx);

        let hierarchy_result = sqlx::query(
            "
INSERT INTO hierarchical_items 
    (parent_folder_id, child_id, sort_order, created_at)
VALUES 
    (?, ?, ?, ?)",
        )
        .bind(model.item.parent_folder_id)
        .bind(folder.item.id)
        .bind(model.item.sort_order)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await;

        tx_check!(hierarchy_result, tx);

        tx.commit().await?;

        Ok(folder)
    }

    async fn create_content(&self, model: CreateContentModel) -> AppResult<Content> {
        let content = Content {
            item: Item {
                id: Uuid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
            kind: model.kind,
            content_path: model.content_path,
            thumbnail_path: model.thumbnail_path,
        };

        let mut tx = self.db.begin().await?;

        let item_result = sqlx::query(
            "INSERT INTO items (id, title, description, kind, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(content.item.id)
        .bind(&content.item.title)
        .bind(&content.item.description)
        .bind(ItemKind::Content)
        .bind(content.item.created_at)
        .execute(&mut *tx)
        .await;

        tx_check!(item_result, tx);

        let content_result = sqlx::query(
            "INSERT INTO contents (item_id, kind, content_path, thumbnail_path) VALUES (?, ?, ?, ?)"
        )
        .bind(content.item.id)
        .bind(content.kind)
        .bind(&content.content_path)
        .bind(&content.thumbnail_path)
        .execute(&mut *tx)
        .await;

        tx_check!(content_result, tx);

        let hierarchy_result = sqlx::query(
            "
INSERT INTO hierarchical_items 
    (parent_folder_id, child_id, sort_order, created_at)
VALUES 
    (?, ?, ?, ?)",
        )
        .bind(model.item.parent_folder_id)
        .bind(content.item.id)
        .bind(model.item.sort_order)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await;

        tx_check!(hierarchy_result, tx);

        tx.commit().await?;

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use improvie_domain::repository::items::ItemsRepository;
    use improvie_logic::{
        model::items::{FolderNode, ItemNode},
        uuid, Uuid,
    };

    use crate::{
        persistence::db::DbPool,
        repository::{items::ItemsRepositoryImpl, MIGRATOR},
    };

    #[sqlx::test(migrator = "MIGRATOR", fixtures("get_items_hierarchy"))]
    fn get_items_hierarchy(pool: sqlx::SqlitePool) {
        let repo = ItemsRepositoryImpl::new(DbPool::with_pool(pool));
        let res = repo.get_items_hierarchy_loop(Uuid::nil()).await.unwrap();
        let mut map = HashMap::new();
        map.insert(
            Uuid::nil(),
            FolderNode {
                folder: Uuid::nil(),
                items: vec![
                    ItemNode::Folder {
                        id: uuid!("00000000-0000-0000-0000-000000000002"),
                        sort_order: 2,
                    },
                    ItemNode::Content {
                        id: uuid!("00000000-0000-0000-0000-000000000003"),
                        sort_order: 3,
                    },
                    ItemNode::Content {
                        id: uuid!("00000000-0000-0000-0000-000000000004"),
                        sort_order: 1,
                    },
                ],
            },
        );
        map.insert(
            uuid!("00000000-0000-0000-0000-000000000002"),
            FolderNode {
                folder: uuid!("00000000-0000-0000-0000-000000000002"),
                items: vec![ItemNode::Content {
                    id: uuid!("00000000-0000-0000-0000-000000000005"),
                    sort_order: 1,
                }],
            },
        );
        assert_eq!(res, map)
    }
}
