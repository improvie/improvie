use uuid::Uuid;

pub struct Content {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub seconds: u64,
}
