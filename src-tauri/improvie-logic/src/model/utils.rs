#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("utils.ts"))]
pub enum RangeLimit {
    None,
    Min(u32),
    Max(u32),
    MinMax(u32, u32),
}

impl RangeLimit {
    pub const fn is_none(&self) -> bool {
        matches!(self, RangeLimit::None)
    }

    pub const fn as_limit(&self) -> (Option<u32>, Option<u32>) {
        match self {
            RangeLimit::None => (None, None),
            RangeLimit::Min(min) => (Some(*min), None),
            RangeLimit::Max(max) => (None, Some(*max)),
            RangeLimit::MinMax(min, max) => (Some(*min), Some(*max)),
        }
    }
}

#[cfg(feature = "db")]
impl RangeLimit {
    pub fn into_db_condition<C: sea_orm::ColumnTrait>(&self, column: C) -> sea_orm::Condition {
        use sea_orm::sea_query::IntoCondition;

        let (min, max) = self.as_limit();
        match (min, max) {
            (None, None) => sea_orm::Condition::all(),
            (Some(min), None) => column.gte(min).into_condition(),
            (None, Some(max)) => column.lte(max).into_condition(),
            (Some(min), Some(max)) => column.between(min, max).into_condition(),
        }
    }
}
