use std::collections::HashMap;

use chrono::Utc;
use improvie_domain::{
    model::items::{CreateContentModel, CreateFolderModel},
    repository::items::ItemsRepository,
};
use improvie_logic::{
    DynAppResult,
    constant::items::ItemKind,
    model::items::{Content, Folder, FolderNode, Item, ItemNode},
};
use more_convert::VecInto;
use sqlx::QueryBuilder;
use uid::Uid;

use crate::{
    DbErr,
    model::items::{ContentRaw, CurrentNodeRaw, FolderRaw, NodeRaw},
    persistence::db::DbTx,
};

use super::{def_repository_impl, tx_check};

def_repository_impl!(ItemsRepositoryImpl);

#[async_trait::async_trait]
impl ItemsRepository for ItemsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnection<'a>;

    async fn get_items_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<FolderNode> {
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
        .await
        .map_err(DbErr)?;

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
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, FolderNode>> {
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
        .await
        .map_err(DbErr)?;

        let mut nodes: HashMap<Uid, FolderNode> = HashMap::new();
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

    async fn get_contents(&self) -> DynAppResult<Vec<Content>> {
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
        .await
        .map_err(DbErr)?;

        Ok(row.vec_into())
    }

    async fn get_folders(&self) -> DynAppResult<Vec<Folder>> {
        let row: Vec<FolderRaw> = sqlx::query_as(
            "
SELECT
    i.id, i.title, i.description, i.created_at
FROM folders AS f
INNER JOIN items AS i ON f.item_id = i.id
",
        )
        .fetch_all(&self.db.pool())
        .await
        .map_err(DbErr)?;

        Ok(row.vec_into())
    }

    async fn create_folder(&self, model: CreateFolderModel) -> DynAppResult<Folder> {
        let folder = Folder {
            item: Item {
                id: Uid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
        };

        let mut tx = self.db.begin().await?;

        add_item(&mut tx, &folder.item, ItemKind::Folder).await?;

        let folder_result = sqlx::query("INSERT INTO folders (item_id) VALUES (?)")
            .bind(folder.item.id)
            .execute(tx.as_mut())
            .await;

        tx_check!(folder_result, tx);

        add_hierarchy(&mut tx, model.item.parent_folder_id, folder.item.id).await?;

        tx.commit().await?;

        Ok(folder)
    }

    async fn create_content(&self, model: CreateContentModel) -> DynAppResult<Content> {
        let content = Content {
            item: Item {
                id: Uid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
            kind: model.kind,
            content_path: model.content_path,
            thumbnail_path: model.thumbnail_path,
        };

        let mut tx = self.db.begin().await?;

        add_item(&mut tx, &content.item, ItemKind::Content).await?;

        let content_result = sqlx::query(
            "INSERT INTO contents (item_id, kind, content_path, thumbnail_path) VALUES (?, ?, ?, ?)"
        )
        .bind(content.item.id)
        .bind(content.kind)
        .bind(&content.content_path)
        .bind(&content.thumbnail_path)
        .execute(tx.as_mut())
        .await;

        tx_check!(content_result, tx);

        add_hierarchy(&mut tx, model.item.parent_folder_id, content.item.id).await?;

        tx.commit().await?;

        Ok(content)
    }

    async fn delete_item(&self, item_id: Uid) -> DynAppResult<Vec<Uid>> {
        let mut tx = self.db.begin().await?;

        let mut item_uids = sqlx::query_scalar::<_, Uid>(
            "
WITH RECURSIVE item_hierarchy(child_id) AS (
    SELECT
        hi.child_id
    FROM hierarchical_items AS hi
    WHERE hi.parent_folder_id = ?

    UNION ALL

    SELECT
        hi.child_id
    FROM hierarchical_items AS hi
    INNER JOIN item_hierarchy AS ih ON hi.parent_folder_id = ih.child_id
)
SELECT child_id
FROM item_hierarchy
",
        )
        .bind(item_id)
        .fetch_all(tx.as_mut())
        .await
        .map_err(DbErr)?;

        item_uids.push(item_id);

        let mut builder = QueryBuilder::new(
            "
DELETE FROM items
WHERE id IN (
",
        );
        let mut separated = builder.separated(", ");
        for id in &item_uids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");

        builder.build().execute(tx.as_mut()).await.map_err(DbErr)?;

        tx.commit().await?;

        Ok(item_uids)
    }

    async fn update_item_name(&self, item_id: Uid, new_name: String) -> DynAppResult<()> {
        let mut tx = self.db.begin().await?;
        let result = sqlx::query(
            "
UPDATE items
SET title = ?
WHERE id = ?
",
        )
        .bind(&new_name)
        .bind(item_id)
        .execute(tx.as_mut())
        .await;

        tx_check!(result, tx);

        tx.commit().await?;

        Ok(())
    }
}

async fn add_item(tx: &mut DbTx, item: &Item, kind: ItemKind) -> DynAppResult<()> {
    let item_result = sqlx::query(
        "INSERT INTO items (id, title, description, kind, created_at) VALUES (?, ?, ?, ?, ?)",
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

async fn add_hierarchy(tx: &mut DbTx, parent_folder_id: Uid, item_id: Uid) -> DynAppResult<()> {
    let sort_order: u32 = sqlx::query_scalar(
        "
SELECT
    MAX(sort_order)
FROM hierarchical_items
WHERE parent_folder_id = ?
",
    )
    .bind(parent_folder_id)
    .fetch_one(tx.as_mut())
    .await
    .map_err(DbErr)?;

    let sort_order = sort_order + 1;

    let shift_result = sqlx::query(
        "
UPDATE hierarchical_items
SET sort_order = sort_order + 1
WHERE parent_folder_id = ? AND sort_order >= ?
",
    )
    .bind(parent_folder_id)
    .bind(sort_order)
    .execute(tx.as_mut())
    .await;

    shift_result.map_err(DbErr)?;

    let hierarchy_result = sqlx::query(
        "
INSERT INTO hierarchical_items
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use uid::uid;

    use improvie_domain::repository::items::ItemsRepository;
    use improvie_logic::model::items::{FolderNode, ItemNode};
    use uid::Uid;

    use crate::{
        persistence::db::DbPool,
        repository::{MIGRATOR, items::ItemsRepositoryImpl},
    };

    #[sqlx::test(migrator = "MIGRATOR", fixtures("get_items_hierarchy"))]
    fn get_items_hierarchy(pool: sqlx::SqlitePool) {
        let repo = ItemsRepositoryImpl::new(DbPool::with_pool(pool));
        let res = repo.get_items_hierarchy_loop(Uid::nil()).await.unwrap();
        let mut map = HashMap::new();
        map.insert(
            Uid::nil(),
            FolderNode {
                folder: Uid::nil(),
                items: vec![
                    ItemNode::Folder {
                        id: uid!("00000000-0000-0000-0000-000000000002"),
                        sort_order: 2,
                    },
                    ItemNode::Content {
                        id: uid!("00000000-0000-0000-0000-000000000003"),
                        sort_order: 3,
                    },
                    ItemNode::Content {
                        id: uid!("00000000-0000-0000-0000-000000000004"),
                        sort_order: 1,
                    },
                ],
            },
        );
        map.insert(
            uid!("00000000-0000-0000-0000-000000000002"),
            FolderNode {
                folder: uid!("00000000-0000-0000-0000-000000000002"),
                items: vec![ItemNode::Content {
                    id: uid!("00000000-0000-0000-0000-000000000005"),
                    sort_order: 1,
                }],
            },
        );
        assert_eq!(res, map)
    }
}
