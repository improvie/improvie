import type { YtVideoRequest } from '$bindings/yt';
import type { Helpers } from 'youtubei.js/web';
import { invoke } from '@tauri-apps/api/core';
import Innertube, { UniversalCache, YTNodes } from 'youtubei.js/web';
import { generatePoToken } from './potoken';

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

  video_formats: YtFormat[];
  best_audio: YtFormat;
}

export interface YtFormat {
  type: 'audio' | 'video';
  quality: string;
  mime_type: string;
  // only used for video format
  quality_label: string | undefined;
  url: string;
}

let client: Innertube | undefined;

async function getClient(forceInit?: boolean): Promise<Innertube> {
  if (forceInit || !client) {
    const data = await generatePoToken();
    client = await Innertube.create({
      po_token: data.poToken,
      visitor_data: data.visitorData,
      cache: new UniversalCache(false),
      generate_session_locally: true,
    });
  }
  return client;
}

export async function getVideoDetail(videoId: string): Promise<VideoDetail> {
  const client = await getClient();
  const videoInfo = await client.getBasicInfo(videoId, 'IOS');
  const videoInfoByMWEBPromise = client.getBasicInfo(videoId, 'MWEB');

  if (videoInfo.playability_status?.status !== 'OK') {
    if (videoInfo.playability_status?.status === 'UNPLAYABLE') {
      throw new Error(`Video is unplayable: ${videoInfo.playability_status.reason || 'Unknown reason'}`);
    }
    if (videoInfo.playability_status?.status === 'LOGIN_REQUIRED') {
      await getClient(true); // Force re-initialization to refresh session
      throw new Error('Blocked by YouTube: Session refreshed, please try again.');
    }
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
        url: format.decipher(client.session.player),
      };
    });

  const video_formats: YtFormat[] = getGoodVideos(formats);
  if (video_formats.length === 0) {
    throw new Error('No suitable video format found for this video.');
  }

  const bestAudio = getBestAudio(formats);
  if (!bestAudio) {
    throw new Error('No suitable audio format found for this video.');
  }

  const videoInfoByMWEB = await videoInfoByMWEBPromise;

  const thumbnails = [...videoInfo.basic_info?.thumbnail || [], ...videoInfoByMWEB.basic_info?.thumbnail || []];
  const thumbnail_url = getBestThumbnailUrl(thumbnails);

  return {
    thumbnail_url,
    video_id: videoId,
    title,
    video_formats,
    best_audio: bestAudio,
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

  const client = await getClient();

  let playlist: TempPlaylist;
  if (videoId) {
    const endpoint = new YTNodes.NavigationEndpoint({
      NavigationEndpoint: {
        videoId,
        playlistId,
      },
    });
    const info = await client.getInfo(endpoint, 'IOS');
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

  const thumbnail_url = getBestThumbnailUrl(playlist.info.thumbnails);

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

function getBestThumbnailUrl(thumbnails: { url: string; width: number; height: number }[]): string | undefined {
  if (!thumbnails || thumbnails.length === 0) {
    return undefined;
  }
  const bestThumbnail = thumbnails.sort((a, b) => b.width - a.width)[0];
  return bestThumbnail.url;
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

function getVideoQuality(format: YtFormat): number {
  if (!format.quality_label) {
    return 0;
  }
  // quali_label is usually a string like "1080p", "720p", etc.
  const qualityMatch = format.quality_label.match(/(\d+)/);
  if (qualityMatch) {
    return Number.parseInt(qualityMatch[1], 10); // Extract the numeric part
  }
  return 0;
}

export function getGoodVideos(formats: YtFormat[]): YtFormat[] {
  const sorted = formats
    .filter(format => format.type === 'video')
    .sort((a, b) => {
      const aQuality = getVideoQuality(a);
      const bQuality = getVideoQuality(b);
      return bQuality - aQuality;
    });
  const uniqueQualities: Set<string> = new Set();
  return sorted.filter((format) => {
    if (!format.quality_label) {
      return false;
    }
    if (uniqueQualities.has(format.quality_label)) {
      return false;
    }
    uniqueQualities.add(format.quality_label);
    return true;
  });
}

export function extractIdsFromUrl(url: URL): {
  videoId?: string;
  playlistId?: string;
} {
  const videoRegExp = /^.*((youtu.be\/)|(v\/)|(\/u\/\w\/)|(embed\/)|(shorts\/)|(watch\?))\??v?=?([^#&?]*).*/;

  const match = url.toString().match(videoRegExp);
  const videoId = (match && match[8] !== undefined && match[8].length === 11) ? match[8] : undefined;

  const playlistId = url.searchParams.get('list') || undefined;

  return {
    videoId,
    playlistId,
  };
}

export function isMixList(playlistId: string): boolean {
  return playlistId.startsWith('RD');
}

// returns a boolean indicating whether the callback was successfully notified
// use `listen('yt-downloading-state', (event: YtVideoState) => { ... })`
export async function import_youtube_video(request: YtVideoRequest): Promise<boolean> {
  try {
    return await invoke<boolean>('import_youtube_video', {
      request,
    });
  }
  catch (error) {
    console.error('Error importing YouTube video:', error);
    return false;
  }
}
