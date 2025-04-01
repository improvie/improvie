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
    data_dir: PathBuf,
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
        rt.block_on(async move {
            log::info!("Starting youtube integration background task");

            let fetcher = Youtube::with_new_binaries(libraries_dir, output_dir).await;
            match fetcher {
                Ok(fetcher) => {
                    log::info!("Youtube integration ready");
                    let mut arc = arc.write().await;
                    *arc = YtStore::Loaded(Self { fetcher, data_dir });
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

    pub async fn download_content(
        &self,
        url: String,
    ) -> Result<YtContent, Box<dyn std::error::Error>> {
        log::info!("Fetching video info for {}", url);

        let video = self.fetcher.fetch_video_infos(url).await?;

        log::info!("Downloading video {}: {}", video.id, video.title);

        let content_path = self
            .fetcher
            .download_video(&video, format!("{}.mp4", video.id))
            .await?;

        let thumbnail_path = self
            .fetcher
            .download_thumbnail(&video, format!("{}.jpg", video.id))
            .await?;

        Ok(YtContent {
            title: video.title,
            content_path: self.data_dir.join(content_path),
            thumbnail_path: self.data_dir.join(thumbnail_path),
        })
    }
}
