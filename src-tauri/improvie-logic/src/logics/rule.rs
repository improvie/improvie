use uuid::Uuid;

pub enum Rule {
    Content(Uuid),
    Range(Uuid, Option<u64>, Option<u64>),
    Loop(Uuid, u8),
}
