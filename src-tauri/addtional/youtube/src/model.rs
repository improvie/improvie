use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("yt.ts"))]
pub struct YtVideoRequest {
    pub process_id: String,
    // If the file name is not provided, use the process_id as the file name
    pub file_name: Option<String>,

    pub video_url: String,
    pub audio_url: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "ts", bind::ts("yt.ts"))]
#[serde(tag = "type", content = "data")]
pub enum YtVideoState {
    Idle {
        process_id: String,
    },
    Downloading {
        process_id: String,
        state: YtVideoDownloading,
    },
    Completed {
        process_id: String,
        state: YtVideoDownloadComplete,
    },
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
    pub video_path: PathBuf,
    pub thumbnail_path: Option<PathBuf>,
}
