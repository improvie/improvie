use uuid::Uuid;

pub struct CreateBasePlayItemModel {
    pub parent_folder_id: Uuid,

    pub title: String,
    pub description: Option<String>,
}

pub struct CreatePlayFolderModel {
    pub item: CreateBasePlayItemModel,
}

pub struct CreatePlaylistModel {
    pub item: CreateBasePlayItemModel,

    pub thumbnail_path: Option<String>,
}
