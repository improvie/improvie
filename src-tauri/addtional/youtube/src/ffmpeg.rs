use std::path::Path;

pub(crate) async fn movflags_faststart(
    from: &Path,
    to: &Path,
) -> Result<(), ez_ffmpeg::error::Error> {
    log::info!("Processing movflags faststart");
    ez_ffmpeg::FfmpegContext::builder()
        .input(from.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(to.to_string_lossy().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map_with_copy("0:a?")
                .set_format_opt("movflags", "faststart"),
        )
        .build()?
        .start()?
        .await?;
    Ok(())
}

pub(crate) async fn audio_to_acc(from: &Path, to: &Path) -> Result<(), ez_ffmpeg::error::Error> {
    log::info!("Converting audio to acc");
    ez_ffmpeg::FfmpegContext::builder()
        .input(from.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(to.to_string_lossy().to_string())
                .add_stream_map("0:a")
                .set_audio_codec("aac"),
        )
        .build()?
        .start()?
        .await?;
    Ok(())
}

pub(crate) async fn merge_video_audio(
    video_path: &Path,
    audio_path: &Path,
    output_path: &Path,
) -> Result<(), ez_ffmpeg::error::Error> {
    log::info!("Merging video and audio");
    ez_ffmpeg::FfmpegContext::builder()
        .input(video_path.to_string_lossy().to_string())
        .input(audio_path.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(output_path.to_string_lossy().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map_with_copy("1:a?"),
        )
        .build()?
        .start()?
        .await?;
    Ok(())
}
