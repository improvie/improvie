mod error;
pub use error::YtError;

mod model;
pub use model::SingleVideoDownload;
pub use model::YtPlaylistDownloadState;
pub use model::YtVideoDownloadState;

mod ffmpeg;
mod yt;

pub use yt::download_playlist;
pub use yt::download_single_video;
