use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("yt.ts"))]
pub struct YtVideoRequest {
    pub video_id: String,
    pub content_title: String,

    pub video_url: String,
    pub audio_url: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "ts", bind::ts("yt.ts"))]
pub enum YtVideoState {
    Idle,
    Downloading(YtVideoDownloading),
    Completed(YtVideoDownloadComplete),
}

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "ts", bind::ts("yt.ts"))]
pub struct YtVideoDownloading {
    pub downloaded_mb: u64,
    pub total_mb: u64,
    pub percentage: u8,
}

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "ts", bind::ts("yt.ts"))]
pub struct YtVideoDownloadComplete {
    pub title: String,
    pub video_path: PathBuf,
    pub thumbnail_path: Option<PathBuf>,
}
