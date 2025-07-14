<script lang='ts'>
  import type { PlaylistDetail } from '$lib/youtube';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
  import { Logger } from '$lib/logger';
  import { getPlaylistDetail } from '$lib/youtube';
  import { toast } from 'svelte-sonner';
  import YtVideoComponent from './YtVideoComponent.svelte';

  let {
    parent_folder_id,
    playlistId,
    videoId,
    processing = $bindable(),
  }: {
    parent_folder_id: string;
    playlistId: string;
    videoId: string | undefined;
    processing: boolean;
  } = $props();

  let detail: PlaylistDetail | undefined = $state();

  async function init() {
    try {
      detail = await getPlaylistDetail(playlistId, videoId);
    }
    catch (error) {
      Logger.error('Failed to fetch playlist details:', error);
      toast.error('Failed to fetch playlist details. Please check the playlist ID and try again.');
      processing = false;
    }
  }

  $effect(() => {
    init();
  });

  const videos: YtVideoComponent[] = $state([]);

  async function importAllVideos() {
    if (!detail) {
      toast.error('No playlist details available to import videos.');
      return;
    }

    const videoCount = detail.videos.length;
    let importedCount = 0;
    for (const video of videos) {
      video.importVideo().then(() => {
        importedCount++;
        if (importedCount === videoCount) {
          toast.success(`Successfully imported ${videoCount} videos from the playlist.`);
          return;
        }
        if (importedCount % 5 === 0) {
          toast.info(`Imported ${importedCount} of ${videoCount} videos...`);
        }
      }).catch(() => {
        console.error('Error importing video');
        toast.error('Error importing video. Please try again later.');
      });
    }
  }

</script>

{#if detail}
  <div class='flex flex-col gap-2 mb-4'>
    <Button variant='destructive' onclick={importAllVideos}>
      Import {detail.videos.length} videos from playlist
    </Button>
  </div>
  <ScrollArea>
    <div
      class='flex flex-col gap-2 w-full h-max'
      style='max-height: calc(var(--item-height) * 3 + 16px);'
    >
      {#each detail.videos as videoDetail, i}
        <YtVideoComponent
          parent_folder_id={parent_folder_id}
          detail={videoDetail}
          bind:this={videos[i]}
        />
      {/each}
    </div>
  </ScrollArea>
{:else}
  <Dialog.Header>
    <Dialog.Title>Fetching playlist...</Dialog.Title>
    <Dialog.Description>
      Please wait while we fetch the playlist details.
    </Dialog.Description>
  </Dialog.Header>
{/if}
