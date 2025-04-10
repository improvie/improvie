use std::{path::PathBuf, sync::Arc};

use tokio::{runtime::Builder, sync::RwLock};
use yt_dlp::Youtube;

pub struct YtContent {
    pub title: String,
    pub content_path: PathBuf,
    pub thumbnail_path: PathBuf,
}

#[derive(Debug)]
pub enum YtStore {
    Loading,
    Loaded(YtIntegration),
    Error(Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone)]
pub struct YtIntegration {
    fetcher: Youtube,
    #[allow(dead_code)]
    data_dir: PathBuf,
    outputs_dir: PathBuf,
}

impl YtIntegration {
    pub fn new_background(data_dir: PathBuf, arc: Arc<RwLock<YtStore>>) {
        let output_dir = data_dir.join("output");
        let libraries_dir = data_dir.join("libs");

        let rt = match Builder::new_current_thread().enable_all().build() {
            Ok(rt) => rt,
            Err(err) => {
                log::error!("Youtube integration error on creating thread: {}", err);

                tokio::spawn(async move {
                    let mut arc = arc.write().await;
                    *arc = YtStore::Error(Box::new(err));
                    log::error!("Youtube integration errored");
                });
                return;
            }
        };
        let outputs_dir_clone = output_dir.clone();
        rt.block_on(async move {
            log::info!("Starting youtube integration background task");

            let fetcher = Youtube::with_new_binaries(libraries_dir, output_dir).await;
            match fetcher {
                Ok(fetcher) => {
                    log::info!("Youtube integration ready");
                    let mut arc = arc.write().await;
                    *arc = YtStore::Loaded(Self {
                        fetcher,
                        data_dir,
                        outputs_dir: outputs_dir_clone,
                    });
                    log::info!("Youtube integration loaded");
                }
                Err(err) => {
                    log::error!("Youtube integration error: {}", err);
                    let mut arc = arc.write().await;
                    *arc = YtStore::Error(Box::new(err));
                    log::error!("Youtube integration errored");
                }
            }
        });
    }

    pub async fn download_content_with_progress(
        &self,
        url: String,
        progress_callback: impl Fn(YtDownloadState) + Send + Sync + 'static,
    ) -> Result<YtContent, Box<dyn std::error::Error>> {
        log::info!("Fetching video info for {}", url);

        let video = self.fetcher.fetch_video_infos(url).await?;

        log::info!("Downloading video {}: {}", video.id, video.title);

        let fetcher = self.fetcher.clone();

        let thumbnail_video = video.clone();
        let thumbnail_path = format!("{}.jpg", thumbnail_video.id);
        let thumbnail_path_clone = thumbnail_path.clone();
        let join_thumbnail = tokio::task::spawn(async move {
            fetcher
                .download_thumbnail(&thumbnail_video, thumbnail_path_clone)
                .await
        });

        let video_id = video.id.clone();

        let video_path = format!("{}.mp4", video.id);

        let download_id = self
            .fetcher
            .download_video_with_progress(&video, &video_path, move |downloaded, total| {
                let percentage = if total > 0 {
                    (downloaded as f64 / total as f64 * 100.0) as u8
                } else {
                    0
                };
                let downloaded_mb = downloaded as f64 / 1024.0 / 1024.0;
                let downloaded_mb = downloaded_mb.floor() as u64;
                let total_mb = total as f64 / 1024.0 / 1024.0;
                let total_mb = total_mb.floor() as u64;
                log::info!(
                    "Downloading {} Progress: {}/{} MB - ({}%)",
                    video_id,
                    downloaded_mb,
                    total_mb,
                    percentage
                );
                progress_callback(YtDownloadState {
                    downloaded_mb,
                    total_mb,
                    percentage,
                });
            })
            .await?;

        self.fetcher.wait_for_download(download_id).await;

        join_thumbnail.await??;

        Ok(YtContent {
            title: video.title,
            content_path: self.outputs_dir.join(video_path),
            thumbnail_path: self.outputs_dir.join(thumbnail_path),
        })
    }

    #[allow(dead_code)]
    pub async fn download_content(
        &self,
        url: String,
    ) -> Result<YtContent, Box<dyn std::error::Error>> {
        log::info!("Fetching video info for {}", url);

        let video = self.fetcher.fetch_video_infos(url).await?;

        log::info!("Downloading video {}: {}", video.id, video.title);

        let content_path = format!("{}.mp4", video.id);

        self.fetcher.download_video(&video, &content_path).await?;

        let thumbnail_path = format!("{}.jpg", video.id);

        self.fetcher
            .download_thumbnail(&video, &thumbnail_path)
            .await?;

        Ok(YtContent {
            title: video.title,
            content_path: self.outputs_dir.join(content_path),
            thumbnail_path: self.outputs_dir.join(thumbnail_path),
        })
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct YtDownloadState {
    pub downloaded_mb: u64,
    pub total_mb: u64,
    pub percentage: u8,
}
