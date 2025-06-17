use std::{io::Write, path::PathBuf};

use super::stream::ChunkStream;

pub(crate) async fn download_audio(
    client: reqwest::Client,
    audio_url: String,
    audio_path: PathBuf,
) -> Result<(), crate::YtError> {
    let stream = ChunkStream::new(client, audio_url, None, None).await?;

    let mut file = std::fs::File::create(audio_path)?;

    while let Some(chunk) = stream.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}
