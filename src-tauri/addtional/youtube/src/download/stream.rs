use std::sync::LazyLock;

use bytes::Bytes;
use reqwest::header::{HeaderMap, USER_AGENT};
use tokio::sync::RwLock;

const DEFAULT_CHUNK_SIZE: u64 = 1024 * 1024 * 10;
#[allow(clippy::unwrap_used)]
static DEFAULT_HEADERS: LazyLock<HeaderMap> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.101 Safari/537.36".parse().unwrap());

    headers
});

pub(super) struct ChunkStream {
    client: reqwest::Client,
    content_length: u64,
    chunk_size: u64,
    link: String,
    start: RwLock<u64>,
    end: RwLock<u64>,
}

impl ChunkStream {
    pub(super) async fn new(
        client: reqwest::Client,
        link: String,
        content_length: Option<u64>,
        chunk_size: Option<u64>,
    ) -> Result<Self, crate::YtError> {
        let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);

        let start = 0u64;
        let end = start + chunk_size;

        let mut content_length = content_length.unwrap_or(0);

        if content_length == 0 {
            log::info!("Content length is not provided, fetching from URL: {link}");
            content_length = client
                .get(&link)
                .send()
                .await?
                .error_for_status()?
                .content_length()
                .ok_or(crate::YtError::UrlMissing)?;

            if content_length == 0 {
                log::error!("Content length is 0 for URL: {link}");
                return Err(crate::YtError::UrlMissing);
            }
            log::info!("Content length fetched: {content_length}");
        }

        Ok(Self {
            client,
            content_length,
            chunk_size,
            link,
            start: RwLock::new(start),
            end: RwLock::new(end),
        })
    }

    pub(super) async fn chunk(&self) -> Result<Option<Bytes>, crate::YtError> {
        let end = *self.end.read().await;

        if end == 0 {
            return Ok(None);
        }

        if end >= self.content_length {
            let mut end = self.end.write().await;
            *end = 0;
        }

        let end = *self.end.read().await;
        let range_end = if end == 0 {
            "".to_string()
        } else {
            end.to_string()
        };

        let start = *self.start.read().await;

        let range = format!("bytes={start}-{range_end}");
        let response = self
            .client
            .get(&self.link)
            .headers(DEFAULT_HEADERS.clone())
            .header(reqwest::header::RANGE, range)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await?;

        if end != 0 {
            let mut start = self.start.write().await;
            *start = end + 1;
            let mut end = self.end.write().await;
            *end += self.chunk_size;
        }

        Ok(Some(bytes))
    }

    pub(super) fn content_length(&self) -> u64 {
        self.content_length
    }
}
