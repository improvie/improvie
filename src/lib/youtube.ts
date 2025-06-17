import Innertube from 'youtubei.js';

export interface VideoDetail {
  thumbnail_url: string | undefined;
  video_id: string;
  title: string;

  formats: YtFormat[];
}

export interface YtFormat {
  type: 'audio' | 'video';
  quality: string;
  // only used for video format
  quality_label: string | undefined;
  url: string;
}

// eslint-disable-next-line antfu/no-top-level-await
const client = await Innertube.create();

export async function getVideoDetail(videoId: string): Promise<VideoDetail> {
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
