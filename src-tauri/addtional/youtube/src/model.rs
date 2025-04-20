use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("yt.ts"))]
pub struct YtVideoDownloadState {
    pub downloaded_mb: u64,
    pub total_mb: u64,
    pub percentage: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("yt.ts"))]
pub struct YtPlaylistDownloadState {
    pub index: usize,
    pub state: YtVideoDownloadState,
}

#[derive(Debug, Clone)]
pub struct SingleVideoDownload {
    pub title: String,
    pub id: String,
    pub video_path: PathBuf,
    pub thumbnail_path: Option<PathBuf>,
}
