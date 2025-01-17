use improvie_logic::logic::content::{Content, Folder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ItemNode {
    Content(Content),
    Folder(FolderNode),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FolderNode {
    pub folder: Folder,
    pub children: Vec<ItemNode>,
}
