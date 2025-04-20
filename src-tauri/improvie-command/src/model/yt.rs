#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts)]
pub struct YtDownloadState {
    pub downloaded_mb: u64,
    pub total_mb: u64,
    pub percentage: u8,
}
