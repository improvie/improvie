pub mod health_check;
pub mod items;
pub mod plays;

#[derive(sqlx::FromRow, Debug)]
pub struct HierarchyChildRow {
    pub child_id: uuid::Uuid,
}
