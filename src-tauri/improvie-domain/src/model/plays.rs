use uid::Uid;

pub struct CreateBasePlayItemModel {
    pub parent_folder_id: Uid,

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
