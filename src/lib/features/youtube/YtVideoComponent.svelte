<script lang='ts'>
  import type { YtVideoDownloadComplete, YtVideoState } from '$bindings/yt';
  import type { VideoDetail } from '$lib/youtube';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import * as RadioGroup from '$lib/components/ui/radio-group/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
  import { create_content } from '$lib/stores/items/content';
  import { import_youtube_video } from '$lib/youtube';
  import { listen } from '@tauri-apps/api/event';

  const {
    parent_folder_id,
    detail,
  }: {
    parent_folder_id: string;
    detail: VideoDetail;
  } = $props();

  let downloadUrl = $state<string>(detail.video_formats[0].url);

  let started = $state<boolean>(false);
  let process = $state<YtVideoState | undefined>(undefined);

  const unlisten = listen<YtVideoState>('yt-downloading-state', (event) => {
    const payload = event.payload;
    switch (payload.type) {
      case 'Idle':
        if (payload.data.process_id !== detail.video_id) {
          return;
        }
        process = payload;
        break;
      case 'Downloading':
        if (payload.data.process_id !== detail.video_id) {
          return;
        }
        process = payload;
        break;
      case 'Completed':
        if (payload.data.process_id !== detail.video_id) {
          return;
        }
        process = payload;
        saveContent(payload.data.state);
        break;
    }
  });

  async function saveContent(request: YtVideoDownloadComplete) {
    await create_content({
      kind: 'Video',
      content_path: request.video_path,
      thumbnail_path: request.thumbnail_path ?? null,
      parent_folder_id,
      title: detail.title,
      description: null,
    });
  }

  $effect(() => {
    return () => {
      unlisten.then((fn) => {
        fn();
      });
      process = undefined;
    };
  });

  function importVideo() {
    if (started) {
      return;
    }
    started = true;
    const selectFormat = detail.video_formats.find(
      format => format.url === downloadUrl,
    );
    const quality = selectFormat?.quality_label || 'xxxp';
    import_youtube_video({
      process_id: detail.video_id,
      file_name: `${detail.video_id}-${quality}`,
      video_url: downloadUrl,
      audio_url: detail.best_audio.url,
      thumbnail_url: detail.thumbnail_url ?? null,
    });
  }

</script>

<Card.Root class='sm:max-w-md flex flex-row p-4 h-52'>
  <div class='flex flex-col gap-2'>
    <ImageLoader
      src={detail.thumbnail_url}
      alt={detail.title}
      direction='vertical'
      class='h-32'
      lazy
    />
    <p class='text-sm text-wrap line-clamp-2'>
      {detail.title}
    </p>
  </div>
  <div class='flex flex-col justify-between'>
    {#if !started}
      <ScrollArea class='h-32'>
        <RadioGroup.Root bind:value={downloadUrl} class='flex flex-col gap-2'>
          {#each detail.video_formats as format}
            <div class='flex items-center space-x-2'>
              <RadioGroup.Item value={format.url} id={format.url} />
              <Label for={format.url}>{format.quality_label}</Label>
            </div>
          {/each}
        </RadioGroup.Root>
      </ScrollArea>
      <Button
        onclick={() => {
          importVideo();
        }}
      >
        Import Video
      </Button>
    {:else}
      <div class='flex flex-col gap-2'>
        {#if process}
          <p class='text-sm'>Status: {process.type}</p>
          {#if process.type === 'Downloading'}
            <p class='text-sm'>Progress: {process.data.state.percentage}%</p>
          {/if}
        {:else}
          <p class='text-sm'>Importing...</p>
        {/if}
      </div>
    {/if}
  </div>
</Card.Root>
