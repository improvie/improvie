use std::path::Path;

pub(crate) async fn merge_video_audio(
    video_path: &Path,
    audio_path: &Path,
    output_path: &Path,
) -> Result<(), ez_ffmpeg::error::Error> {
    ez_ffmpeg::FfmpegContext::builder()
        .input(video_path.to_string_lossy().to_string())
        .input(audio_path.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(output_path.to_string_lossy().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map("1:a?")
                .set_audio_codec("aac")
                .set_format_opt("movflags", "faststart"),
        )
        .build()?
        .start()?
        .await?;
    Ok(())
}
