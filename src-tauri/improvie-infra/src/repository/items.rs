use std::collections::HashMap;

use improvie_domain::repository::items::ItemsRepository;
use improvie_logic::{
    constant::items::ItemKind,
    model::items::{Content, Folder, FolderNode, ItemNode},
    util::into::VecInto,
    AppError, AppResult, Uuid,
};
use itertools::Itertools;

use crate::model::items::{ContentRaw, FolderRaw, NodeRaw};

use super::def_repository_impl;

def_repository_impl!(ItemsRepositoryImpl);

#[async_trait::async_trait]
impl ItemsRepository for ItemsRepositoryImpl {
    async fn get_items_hierarchy(&self, folder_id: Uuid) -> AppResult<FolderNode> {
        let row = sqlx::query_as::<_, NodeRaw>(
            "
WITH RECURSIVE folder_hierarchy(depth, parent_folder_id, child_id, child_kind, sort_order) AS (
    SELECT
        0 AS depth,
        hi.parent_folder_id,
        hi.child_id,
        i.kind AS child_kind,
        hi.sort_order
    FROM hierarchical_items AS hi
    INNER JOIN items AS i ON i.id = hi.child_id
    WHERE hi.parent_folder_id = ?

    UNION ALL

    SELECT
        fh.depth + 1 AS depth,
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

        if row.is_empty() {
            return Err(AppError::NotFound("folder", folder_id.to_string()));
        }

        let deps = row
            .into_iter()
            .into_group_map_by(|v| v.depth)
            .into_iter()
            .sorted_by(|a, b| b.0.cmp(&a.0));

        let mut map = HashMap::<Uuid, Vec<ItemNode>>::new();

        for (depth, mut items) in deps {
            let mut lookup = HashMap::new();

            items.sort_by_key(|v| v.sort_order);

            for item in items {
                let val = match item.child_kind {
                    ItemKind::Folder => ItemNode::Folder(FolderNode {
                        folder: item.child_id,
                        items: map.remove(&item.child_id).unwrap_or_default(),
                    }),
                    ItemKind::Content => ItemNode::Content(item.child_id),
                };
                lookup
                    .entry(item.parent_folder_id)
                    .or_insert_with(Vec::new)
                    .push(val);
            }

            if depth == 0 {
                return Ok(FolderNode {
                    folder: folder_id,
                    items: lookup.remove(&folder_id).unwrap_or_default(),
                });
            }

            map = lookup;
        }

        unreachable!()
    }

    async fn get_contents(&self) -> AppResult<Vec<Content>> {
        let row: Vec<ContentRaw> = sqlx::query_as(
            "
SELECT 
    i.id, i.title, i.description, i.created_at,
    c.seconds, c.kind, c.item_path AS path
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
}

#[cfg(test)]
mod tests {
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
        let res = repo.get_items_hierarchy(Uuid::nil()).await.unwrap();
        assert_eq!(
            res,
            FolderNode {
                folder: Uuid::nil(),
                items: vec![
                    ItemNode::Content(uuid!("00000000-0000-0000-0000-000000000004")),
                    ItemNode::Folder(FolderNode {
                        folder: uuid!("00000000-0000-0000-0000-000000000002"),
                        items: vec![ItemNode::Content(uuid!(
                            "00000000-0000-0000-0000-000000000005"
                        ))],
                    }),
                    ItemNode::Content(uuid!("00000000-0000-0000-0000-000000000003"))
                ]
            }
        )
    }
}
