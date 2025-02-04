use improvie_logic::{
    logic::rule::Rule,
    model::playlist::{Playlist, PlaylistFolder},
    Uuid,
};
use more_convert::Convert;

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(PlaylistFolder))]
pub struct PlaylistFolderRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,

    #[convert(ignore)]
    pub parent_id: Uuid,
    pub sort_order: u32,
}

#[derive(sqlx::FromRow, Convert)]
#[convert(into(Playlist))]
pub struct PlaylistRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_path: Option<String>,
    #[sqlx(json)]
    pub rules: Vec<Rule>,

    #[convert(ignore)]
    pub folder_id: Uuid,
    pub sort_order: u32,
}
