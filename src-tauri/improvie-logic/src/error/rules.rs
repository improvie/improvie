pub type RuleResult<T> = Result<T, RuleError>;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Rule")]
pub enum RuleError {
    #[error("not found current rule")]
    NotFoundCurrent,
}
