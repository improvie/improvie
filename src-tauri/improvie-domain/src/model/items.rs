use improvie_logic::constant::items::ContentKind;

pub struct CreateBaseItemModel {
    pub title: String,
    pub description: Option<String>,
}

pub struct CreateFolderModel {
    pub item: CreateBaseItemModel,
}

pub struct CreateContentModel {
    pub item: CreateBaseItemModel,
    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}
