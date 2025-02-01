use improvie_logic::{
    logic::rule::Rule,
    model::playlist::{Play, Playlist},
    Uuid,
};
use more_convert::Convert;

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(Playlist))]
pub struct PlaylistsRaw {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_path: Option<String>,
    pub sort_order: u32,
}

#[derive(sqlx::FromRow, Convert)]
#[convert(into(Play))]
pub struct PlaysRaw {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[sqlx(json)]
    pub rules: Vec<Rule>,
    pub sort_order: u32,
}
