use chrono::{DateTime, Utc};
use improvie_logic::{
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayItem, Playlist},
};
use more_convert::Convert;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(PlayItem))]
pub struct PlayItemRaw {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(PlayFolder))]
pub struct PlayFolderRow {
    #[sqlx(flatten)]
    pub item: PlayItemRaw,
}

#[derive(sqlx::FromRow, Convert)]
#[convert(into(Playlist))]
pub struct PlaylistRow {
    #[sqlx(flatten)]
    pub item: PlayItemRaw,
    pub thumbnail_path: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct PlayCurrentNodeRaw {
    pub child_id: Uuid,
    pub child_kind: PlayItemKind,
    pub sort_order: u32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct PlayNodeRaw {
    pub parent_folder_id: Uuid,
    pub child_id: Uuid,
    pub child_kind: PlayItemKind,
    pub sort_order: u32,
}
