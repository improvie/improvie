use std::{io::Write, path::PathBuf};

use crate::YtVideoDownloading;

use super::stream::ChunkStream;

pub(crate) async fn download_video(
    client: reqwest::Client,
    video_url: String,
    download_path: PathBuf,
    callback: impl Fn(YtVideoDownloading) -> bool,
) -> Result<bool, crate::YtError> {
    let stream = ChunkStream::new(client, video_url, None, None).await?;

    let total_size = stream.content_length();

    let mut downloaded: u64 = 0;

    let mut video_file = std::fs::File::create(download_path)?;

    while let Some(chunk) = stream.chunk().await? {
        video_file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        let state = YtVideoDownloading {
            downloaded_mb: downloaded / 1024 / 1024,
            total_mb: total_size / 1024 / 1024,
            percentage: progress as u8,
        };
        if !callback(state) {
            return Ok(false);
        }
    }

    Ok(true)
}
