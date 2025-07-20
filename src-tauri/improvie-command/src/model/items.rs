use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto};
use improvie_logic::constant::items::ContentKind;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("item/request.ts"))]
pub struct CreateContentRequest {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}

impl CreateContentRequest {
    pub fn into_dto(self, seconds: u32) -> CreateContentDto {
        CreateContentDto {
            item: self.item,
            kind: self.kind,
            content_path: self.content_path,
            thumbnail_path: self.thumbnail_path,
            seconds,
        }
    }
}
