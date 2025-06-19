use std::path::PathBuf;

pub(crate) async fn download_thumbnail(
    client: reqwest::Client,
    thumbnail_url: String,
    thumbnail_path: PathBuf,
) -> Result<(), crate::YtError> {
    let bytes = client
        .get(thumbnail_url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    log::debug!("Starting thumbnail download to {:?}", thumbnail_path);
    std::fs::write(thumbnail_path, bytes)?;
    Ok(())
}
