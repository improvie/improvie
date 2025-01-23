use improvie_logic::{
    constant::items::ItemKind,
    model::items::{FolderNode, ItemNode},
    Uuid,
};

#[derive(sqlx::FromRow, Debug)]
pub struct NodeRaw {
    pub depth: u8,
    pub parent_folder_id: Uuid,
    pub child_id: Uuid,
    pub child_kind: ItemKind,
    pub sort_order: u32,
}

impl From<NodeRaw> for ItemNode {
    fn from(value: NodeRaw) -> Self {
        match value.child_kind {
            ItemKind::Folder => ItemNode::Folder(FolderNode {
                folder: value.child_id,
                items: vec![],
            }),
            ItemKind::Content => ItemNode::Content(value.child_id),
        }
    }
}
