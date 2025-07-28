use sea_orm::ColumnTrait;
use sea_orm::FromQueryResult;
use sea_orm::QueryOrder;
use sea_orm::Statement;
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

    async fn get_items_hierarchy_current(
        &self,
        conn: Self::DbConnection<'_>,
        folder_id: Uid,
    ) -> DynAppResult<FolderNode> {
        let rows = row::hierarchical_items::Entity::find()
            .select_only()
            .column(row::hierarchical_items::Column::ChildId)
            .column(row::hierarchical_items::Column::SortOrder)
            .inner_join(row::items::Entity)
            .column_as(row::items::Column::Kind, "child_kind")
            .filter(row::hierarchical_items::Column::ParentFolderId.eq(folder_id))
            .into_model::<CurrentNodeRaw>()
            .all(&conn)
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
        conn: Self::DbConnection<'_>,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, FolderNode>> {
        let rows = NodeRaw::find_by_statement(Statement::from_sql_and_values(
            conn.backend(),
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
        .all(&conn)
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

    async fn get_contents(&self, conn: Self::DbConnection<'_>) -> DynAppResult<Vec<Content>> {
        let rows = row::contents::Entity::find()
            .select_only()
            .column(row::contents::Column::Kind)
            .column(row::contents::Column::ContentPath)
            .column(row::contents::Column::ThumbnailPath)
            .inner_join(row::items::Entity)
            .column(row::items::Column::Id)
            .column(row::items::Column::Title)
            .column(row::items::Column::Description)
            .column(row::items::Column::CreatedAt)
            .column(row::contents::Column::Seconds)
            .into_model::<ContentRaw>()
            .all(&conn)
            .await?;

        Ok(rows.vec_into())
    }

    async fn get_content_by_id(
        &self,
        conn: Self::DbConnection<'_>,
        uid: Uid,
    ) -> DynAppResult<Option<Content>> {
        let row = row::contents::Entity::find()
            .select_only()
            .column(row::contents::Column::Kind)
            .column(row::contents::Column::ContentPath)
            .column(row::contents::Column::ThumbnailPath)
            .inner_join(row::items::Entity)
            .column(row::items::Column::Id)
            .column(row::items::Column::Title)
            .column(row::items::Column::Description)
            .column(row::items::Column::CreatedAt)
            .column(row::contents::Column::Seconds)
            .filter(row::contents::Column::ItemId.eq(uid))
            .into_model::<ContentRaw>()
            .one(&conn)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn get_folders(&self, conn: Self::DbConnection<'_>) -> DynAppResult<Vec<Folder>> {
        let rows = row::folders::Entity::find()
            .select_only()
            .inner_join(row::items::Entity)
            .column(row::items::Column::Id)
            .column(row::items::Column::Title)
            .column(row::items::Column::Description)
            .column(row::items::Column::CreatedAt)
            .into_model::<FolderRaw>()
            .all(&conn)
            .await?;

        Ok(rows.vec_into())
    }

    async fn get_folder_by_id(
        &self,
        conn: Self::DbConnection<'_>,
        uid: Uid,
    ) -> DynAppResult<Option<Folder>> {
        let row = row::folders::Entity::find()
            .select_only()
            .inner_join(row::items::Entity)
            .column(row::items::Column::Id)
            .column(row::items::Column::Title)
            .column(row::items::Column::Description)
            .column(row::items::Column::CreatedAt)
            .filter(row::folders::Column::ItemId.eq(uid))
            .into_model::<FolderRaw>()
            .one(&conn)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn create_folder(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreateFolderModel,
    ) -> DynAppResult<Folder> {
        let folder = Folder {
            item: Item {
                id: Uid::new(),
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
                id: Uid::new(),
                title: model.item.title,
                description: model.item.description,
                created_at: Utc::now(),
            },
            kind: model.kind,
            content_path: model.content_path,
            thumbnail_path: model.thumbnail_path,
            seconds: model.seconds,
        };

        add_item(conn, &content.item, ItemKind::Content).await?;

        let content_result = improvie_row::contents::Entity::insert(row::contents::ActiveModel {
            item_id: sea_orm::Set(content.item.id),
            kind: sea_orm::Set(content.kind),
            content_path: sea_orm::Set(content.content_path.clone()),
            thumbnail_path: sea_orm::Set(content.thumbnail_path.clone()),
            seconds: sea_orm::Set(content.seconds),
        })
        .exec_without_returning(&conn)
        .await;

        insert_check!(content_result);

        add_hierarchy(conn, model.item.parent_folder_id, content.item.id).await?;

        Ok(content)
    }

    async fn delete_items(
        &self,
        conn: Self::DbConnection<'_>,
        uids: Vec<Uid>,
    ) -> DynAppResult<Vec<(Uid, ItemKind)>> {
        let result = row::items::Entity::find()
            .select_only()
            .column(row::items::Column::Id)
            .column(row::items::Column::Kind)
            .filter(row::items::Column::Id.is_in(uids))
            .into_tuple::<(Uid, ItemKind)>()
            .all(&conn)
            .await?;

        let mut flattened = Vec::with_capacity(result.len());

        for (uid, kind) in result {
            match kind {
                ItemKind::Content => flattened.push((uid, ItemKind::Content)),
                ItemKind::Folder => {
                    let nodes = self.get_items_hierarchy_loop(conn, uid).await?;
                    for v in nodes.values() {
                        flattened.push((v.folder, ItemKind::Folder));
                        for v in &v.items {
                            match v {
                                ItemNode::Folder { id, .. } => {
                                    flattened.push((*id, ItemKind::Folder))
                                }
                                ItemNode::Content { id, .. } => {
                                    flattened.push((*id, ItemKind::Content))
                                }
                            }
                        }
                    }
                }
            }
        }

        row::items::Entity::delete_many()
            .filter(
                row::items::Column::Id
                    .is_in(flattened.iter().map(|(uid, _)| *uid).collect::<Vec<Uid>>()),
            )
            .exec(&conn)
            .await?;

        Ok(flattened)
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
        .column(row::hierarchical_items::Column::SortOrder)
        .filter(row::hierarchical_items::Column::ParentFolderId.eq(parent_folder_id))
        .order_by_desc(row::hierarchical_items::Column::SortOrder)
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
