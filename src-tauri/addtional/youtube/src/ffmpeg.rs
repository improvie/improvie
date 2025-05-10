use std::path::Path;

use crate::YtError;

#[cfg(feature = "include")]
macro_rules! try_ffmpeg {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(YtError::Ffmpeg(Box::new(e))),
        }
    };
}

#[cfg(not(feature = "include"))]
pub(crate) async fn movflags_faststart(_from: &Path, _to: &Path) -> Result<(), YtError> {
    unimplemented!("movflags_faststart is not supported without the `include` feature");
}

#[cfg(feature = "include")]
pub(crate) async fn movflags_faststart(from: &Path, to: &Path) -> Result<(), YtError> {
    log::info!("Processing movflags faststart");
    let build = ez_ffmpeg::FfmpegContext::builder()
        .input(from.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(to.to_string_lossy().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map_with_copy("0:a?")
                .set_format_opt("movflags", "faststart"),
        )
        .build();
    let start = try_ffmpeg!(build).start();
    let result = try_ffmpeg!(start).await;
    try_ffmpeg!(result);
    log::info!("Finished processing movflags faststart");
    Ok(())
}

#[cfg(not(feature = "include"))]
pub(crate) async fn audio_to_acc(_from: &Path, _to: &Path) -> Result<(), YtError> {
    unimplemented!("audio_to_acc is not supported without the `include` feature");
}

#[cfg(feature = "include")]
pub(crate) async fn audio_to_acc(from: &Path, to: &Path) -> Result<(), YtError> {
    log::info!("Converting audio to acc");
    let build = ez_ffmpeg::FfmpegContext::builder()
        .input(from.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(to.to_string_lossy().to_string())
                .add_stream_map("0:a")
                .set_audio_codec("aac"),
        )
        .build();
    let start = try_ffmpeg!(build).start();
    let result = try_ffmpeg!(start).await;
    try_ffmpeg!(result);
    Ok(())
}

#[cfg(not(feature = "include"))]
pub(crate) async fn merge_video_audio(
    _video_path: &Path,
    _audio_path: &Path,
    _output_path: &Path,
) -> Result<(), YtError> {
    unimplemented!("merge_video_audio is not supported without the `include` feature");
}

#[cfg(feature = "include")]
pub(crate) async fn merge_video_audio(
    video_path: &Path,
    audio_path: &Path,
    output_path: &Path,
) -> Result<(), YtError> {
    log::info!("Merging video and audio");
    let build = ez_ffmpeg::FfmpegContext::builder()
        .input(video_path.to_string_lossy().to_string())
        .input(audio_path.to_string_lossy().to_string())
        .output(
            ez_ffmpeg::Output::new(output_path.to_string_lossy().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map_with_copy("1:a?"),
        )
        .build();
    let start = try_ffmpeg!(build).start();
    let result = try_ffmpeg!(start).await;
    try_ffmpeg!(result);
    Ok(())
}
