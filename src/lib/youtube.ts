import type { YtVideoRequest } from '$bindings/yt';
import type { Helpers } from 'youtubei.js';
import { invoke } from '@tauri-apps/api/core';
import Innertube from 'youtubei.js';
import { YTNodes } from 'youtubei.js';

export interface PlaylistDetail {
  playlist_id: string;
  title: string;
  thumbnail_url: string | undefined;
  videos: VideoDetail[];
}

export interface VideoDetail {
  video_id: string;
  title: string;
  thumbnail_url: string | undefined;

  formats: YtFormat[];
}

export interface YtFormat {
  type: 'audio' | 'video';
  quality: string;
  mime_type: string;
  // only used for video format
  quality_label: string | undefined;
  url: string;
}

let client: Innertube | undefined = $state(undefined);

export async function getVideoDetail(videoId: string): Promise<VideoDetail> {
  if (!client) {
    client = await Innertube.create();
  }
  const player = client.actions.session.player;
  if (player === undefined) {
    throw new Error('Player is not defined in the session. This usually means that the player script could not be loaded.');
  }

  const videoInfo = await client.getInfo(videoId, 'MWEB');

  if (videoInfo.playability_status?.status !== 'OK') {
    throw new Error(`Video is not playable: ${videoInfo.playability_status?.reason || 'Unknown reason'}`);
  }

  if (!videoInfo.streaming_data) {
    throw new Error('No streaming data available for this video.');
  }

  if (!videoInfo.streaming_data.formats && !videoInfo.streaming_data.adaptive_formats) {
    throw new Error('No formats available for this video.');
  }

  if (!videoInfo.basic_info.title) {
    throw new Error('No title available for this video.');
  }

  const title = videoInfo.basic_info.title;

  const allFormats = [...videoInfo.streaming_data?.formats || [], ...videoInfo.streaming_data?.adaptive_formats || []];

  const cpn = videoInfo.cpn;

  const formats: YtFormat[] = allFormats
    .filter((format) => {
      if (format.has_video && format.has_audio) {
        return false;
      }
      if (!format.has_video && !format.has_audio) {
        return false;
      }
      if (format.quality === undefined) {
        return false;
      }
      return true;
    })
    .map((format) => {
      return {
        type: format.has_video ? 'video' : 'audio',
        mime_type: format.mime_type,
        quality: format.quality!,
        quality_label: format.quality_label,
        url: `${format.decipher(player)}&cpn=${cpn}`,
      };
    });

  const thumbnail_url = videoInfo.basic_info?.thumbnail?.sort((a, b) => b.width - a.width)[0]?.url;

  return {
    thumbnail_url,
    video_id: videoId,
    title,
    formats,
  };
}

export async function getPlaylistDetail(playlistId: string, videoId?: string): Promise<PlaylistDetail> {
  interface TempPlaylist {
    info: {
      title?: string | undefined;
      thumbnails: {
        url: string;
        width: number;
        height: number;
      }[];
    };
    items: Helpers.YTNode[];
  }

  if (!client) {
    client = await Innertube.create();
  }

  let playlist: TempPlaylist;
  if (videoId) {
    const endpoint = new YTNodes.NavigationEndpoint({
      NavigationEndpoint: {
        videoId,
        playlistId,
      },
    });
    const info = await client.getInfo(endpoint, 'MWEB');
    const playlistInfo = info.playlist;
    if (!playlistInfo) {
      throw new Error('No playlist information available for this video.');
    }
    playlist = {
      info: {
        title: playlistInfo.title,
        thumbnails: [],
      },
      items: playlistInfo.contents,
    };
  }
  else {
    playlist = await client.getPlaylist(playlistId);
  }
  const title = playlist.info.title;

  if (!title) {
    throw new Error('No title available for this playlist.');
  }

  const thumbnail_url = playlist.info.thumbnails?.sort((a, b) => b.width - a.width)[0]?.url;

  const promises: Promise<VideoDetail>[] = [];
  for (const video of playlist.items) {
    if ('video_id' in video && typeof video.video_id === 'string') {
      const videoId = video.video_id;
      promises.push(getVideoDetail(videoId));
    }
  }
  const videos: VideoDetail[] = await Promise.all(promises);

  return {
    playlist_id: playlistId,
    title,
    thumbnail_url,
    videos,
  };
}

function getCodecsFromMimeType(mimeType: string): string[] {
  if (!mimeType.includes('codecs=')) {
    return [];
  }
  const params = mimeType.split(';');
  if (params.length < 2) {
    return [];
  }
  const codecsParam = params.find(param => param.trim().startsWith('codecs='));
  if (!codecsParam) {
    return [];
  }
  let codecs = codecsParam.split('=')[1];
  if (codecs.startsWith('"') && codecs.endsWith('"')) {
    codecs = codecs.slice(1, -1); // Remove quotes if present
  }
  return codecs.split(',').map(codec => codec.trim());
}

export function getBestAudio(formats: YtFormat[]): YtFormat | undefined {
  const AUDIO_ENCODING_RANKS: string[] = ['mp4a', 'mp3', 'vorbis', 'aac', 'opus', 'flac'];
  return formats
    .filter(format => format.type === 'audio')
    .sort((a, b) => {
      const aCodecs = getCodecsFromMimeType(a.mime_type);
      const bCodecs = getCodecsFromMimeType(b.mime_type);
      const aRank = AUDIO_ENCODING_RANKS.findIndex(codec => aCodecs.includes(codec));
      const bRank = AUDIO_ENCODING_RANKS.findIndex(codec => bCodecs.includes(codec));
      if (aRank === -1 && bRank === -1) {
        return 0; // Both formats have no recognized audio codec
      }
      if (aRank === -1) {
        return 1; // a has no recognized audio codec, b is better
      }
      if (bRank === -1) {
        return -1; // b has no recognized audio codec, a is better
      }
      return aRank - bRank; // Compare ranks
    })[0];
}

// use `listen('yt-downloading-state', (event: YtVideoState) => { ... })`
export async function import_youtube_video(request: YtVideoRequest): Promise<boolean> {
  try {
    await invoke('import_youtube_video', request);
    return true;
  }
  catch (error) {
    console.error('Error importing YouTube video:', error);
    return false;
  }
}
