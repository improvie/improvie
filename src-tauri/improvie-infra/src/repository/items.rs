use sea_orm::ColumnTrait;
use sea_orm::FromQueryResult;
use sea_orm::Statement;
use sea_orm::TryGetableMany;
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
use sea_orm::{EntityTrait, QueryFilter, QuerySelect};
use uid::Uid;

use crate::model::items::{ContentRaw, CurrentNodeRaw, FolderRaw, NodeRaw};
use crate::repository::modify_check;

use super::{def_repository_impl, insert_check};

use improvie_row as row;

def_repository_impl!(ItemsRepositoryImpl);

#[async_trait::async_trait]
impl ItemsRepository for ItemsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnectionImpl<'a>;

    async fn get_items_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<FolderNode> {
        let rows = row::hierarchical_items::Entity::find()
            .select_only()
            .column(row::hierarchical_items::Column::ChildId)
            .column(row::hierarchical_items::Column::SortOrder)
            .left_join(row::items::Entity)
            .column(row::items::Column::Kind)
            .filter(row::hierarchical_items::Column::ParentFolderId.eq(folder_id))
            .into_model::<CurrentNodeRaw>()
            .all(self.db.pool())
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
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, FolderNode>> {
        let rows = NodeRaw::find_by_statement(Statement::from_sql_and_values(
            self.db.backend(),
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
            [folder_id.into()],
        ))
        .all(self.db.pool())
        .await?;

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
        let rows = row::contents::Entity::find()
            .select_only()
            .column(row::contents::Column::Kind)
            .column(row::contents::Column::ContentPath)
            .column(row::contents::Column::ThumbnailPath)
            .left_join(row::items::Entity)
            .column(row::items::Column::Id)
            .column(row::items::Column::Title)
            .column(row::items::Column::Description)
            .column(row::items::Column::CreatedAt)
            .into_model::<ContentRaw>()
            .all(self.db.pool())
            .await?;

        Ok(rows.vec_into())
    }

    async fn get_folders(&self) -> DynAppResult<Vec<Folder>> {
        let rows = row::folders::Entity::find()
            .select_only()
            .left_join(row::items::Entity)
            .column(row::items::Column::Id)
            .column(row::items::Column::Title)
            .column(row::items::Column::Description)
            .column(row::items::Column::CreatedAt)
            .into_model::<FolderRaw>()
            .all(self.db.pool())
            .await?;

        Ok(rows.vec_into())
    }

    async fn create_folder(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreateFolderModel,
    ) -> DynAppResult<Folder> {
        let folder = Folder {
            item: Item {
                id: Uid::now(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
        };

        add_item(conn, &folder.item, ItemKind::Folder).await?;

        let folder_result = row::folders::Entity::insert(row::folders::ActiveModel {
            item_id: sea_orm::Set(folder.item.id),
        })
        .exec_without_returning(&conn)
        .await;

        insert_check!(folder_result);

        add_hierarchy(conn, model.item.parent_folder_id, folder.item.id).await?;

        Ok(folder)
    }

    async fn create_content(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreateContentModel,
    ) -> DynAppResult<Content> {
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

        add_item(conn, &content.item, ItemKind::Content).await?;

        let content_result = improvie_row::contents::Entity::insert(row::contents::ActiveModel {
            item_id: sea_orm::Set(content.item.id),
            kind: sea_orm::Set(content.kind),
            content_path: sea_orm::Set(content.content_path.clone()),
            thumbnail_path: sea_orm::Set(content.thumbnail_path.clone()),
        })
        .exec_without_returning(&conn)
        .await;

        insert_check!(content_result);

        add_hierarchy(conn, model.item.parent_folder_id, content.item.id).await?;

        Ok(content)
    }

    async fn delete_item(
        &self,
        conn: Self::DbConnection<'_>,
        item_id: Uid,
    ) -> DynAppResult<Vec<Uid>> {
        let mut item_uids = Uid::find_by_statement::<row::hierarchical_items::Column>(
            Statement::from_sql_and_values(
                self.db.backend(),
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
                [item_id.into()],
            ),
        )
        .all(&conn)
        .await?;

        item_uids.push(item_id);

        let result = row::items::Entity::delete_many()
            .filter(row::items::Column::Id.is_in(item_uids.clone()))
            .exec(&conn)
            .await;

        modify_check!(result);

        Ok(item_uids)
    }

    async fn update_item_name(
        &self,
        conn: Self::DbConnection<'_>,
        item_id: Uid,
        new_name: String,
    ) -> DynAppResult<()> {
        let target = row::items::ActiveModel {
            title: sea_orm::Set(new_name),
            ..Default::default()
        };
        let result = row::items::Entity::update_many()
            .filter(row::items::Column::Id.eq(item_id))
            .set(target)
            .exec(&conn)
            .await;

        modify_check!(result);

        Ok(())
    }
}

async fn add_item(
    conn: crate::persistence::db::DbConnectionImpl<'_>,
    item: &Item,
    kind: ItemKind,
) -> DynAppResult<()> {
    let item_result = row::items::Entity::insert(row::items::ActiveModel {
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

async fn add_hierarchy(
    conn: crate::persistence::db::DbConnectionImpl<'_>,
    parent_folder_id: Uid,
    item_id: Uid,
) -> DynAppResult<()> {
    let sort_order = row::hierarchical_items::Entity::find()
        .select_only()
        .expr(row::hierarchical_items::Column::SortOrder.into_expr().max())
        .filter(row::hierarchical_items::Column::ParentFolderId.eq(parent_folder_id))
        .into_tuple::<u32>()
        .one(&conn)
        .await?
        .unwrap_or(0);

    let sort_order = sort_order + 1;

    let shift_result = row::hierarchical_items::Entity::update_many()
        .filter(row::hierarchical_items::Column::ParentFolderId.eq(parent_folder_id))
        .filter(row::hierarchical_items::Column::SortOrder.gte(sort_order))
        .col_expr(
            row::hierarchical_items::Column::SortOrder,
            row::hierarchical_items::Column::SortOrder
                .into_expr()
                .add(1),
        )
        .exec(&conn)
        .await;

    // If affecting rows is 0, it means there are no items to shift. not an error.
    shift_result?;

    let hierarchy_result =
        row::hierarchical_items::Entity::insert(row::hierarchical_items::ActiveModel {
            parent_folder_id: sea_orm::Set(parent_folder_id),
            child_id: sea_orm::Set(item_id),
            sort_order: sea_orm::Set(sort_order),
            created_at: sea_orm::Set(Utc::now()),
        })
        .exec_without_returning(&conn)
        .await;

    insert_check!(hierarchy_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use uid::uid;

    use improvie_domain::repository::items::ItemsRepository;
    use improvie_logic::model::items::{FolderNode, ItemNode};
    use uid::Uid;

    use crate::{persistence::db::DbPoolImpl, repository::items::ItemsRepositoryImpl};

    #[tokio::test]
    async fn get_items_hierarchy() {
        let repo = ItemsRepositoryImpl::new(DbPoolImpl::new_test().await);
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
