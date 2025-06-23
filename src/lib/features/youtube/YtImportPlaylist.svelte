<script lang='ts'>
  import type { PlaylistDetail } from '$lib/youtube';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
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
      console.error('Failed to fetch playlist details:', error);
      toast.error('Failed to fetch playlist details. Please check the playlist ID and try again.');
      processing = false;
    }
  }

  $effect(() => {
    init();
  });
</script>

{#if detail}
  <ScrollArea style='max-height: calc(var(--item-height) * 3 + 16px);'>
    <div class='flex flex-col gap-2 w-full h-full'>
      {#each detail.videos as videoDetail}
        <YtVideoComponent
          parent_folder_id={parent_folder_id}
          detail={videoDetail}
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
