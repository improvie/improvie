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
                .add_stream_map_with_copy("1:a?")
                .set_video_codec_opt("preset", "fast")
                .set_format_opt("movflags", "faststart"),
        )
        .build();
    let start = try_ffmpeg!(build).start();
    let result = try_ffmpeg!(start).await;
    try_ffmpeg!(result);
    Ok(())
}

#[cfg(not(feature = "include"))]
pub fn get_duration(_path: String) -> Result<u32, YtError> {
    unimplemented!("get_duration is not supported without the `include` feature");
}

#[cfg(feature = "include")]
pub fn get_duration(path: String) -> Result<u32, YtError> {
    use ez_ffmpeg::container_info::get_duration_us;

    log::info!("Getting duration of video file: {path}");
    let duration_micro = try_ffmpeg!(get_duration_us(path));
    let duration_seconds = duration_micro / 1_000_000;
    if 0 >= duration_seconds {
        return Err(YtError::Ffmpeg(Box::new(ez_ffmpeg::error::Error::IO(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Duration of video file is zero or negative",
            ),
        ))));
    }

    log::info!("Duration of video file: {duration_seconds} seconds");
    Ok(duration_seconds as u32)
}
