<script lang='ts'>
  import type { VideoDetail } from '$lib/youtube';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import { Logger } from '$lib/logger';
  import { getVideoDetail } from '$lib/youtube';
  import { toast } from 'svelte-sonner';
  import YtVideoComponent from './YtVideoComponent.svelte';

  let {
    parent_folder_id,
    videoId,
    processing = $bindable(),
  }: {
    parent_folder_id: string;
    videoId: string;
    processing: boolean;
  } = $props();

  let detail: VideoDetail | undefined = $state();

  async function init() {
    try {
      detail = await getVideoDetail(videoId);
    }
    catch (error) {
      Logger.error('Failed to fetch video details:', error);
      toast.error('Failed to fetch video details. Please check the video ID and try again.');
      processing = false;
    }
  }

  $effect(() => {
    init();
  });
</script>

{#if detail}
  <YtVideoComponent
    parent_folder_id={parent_folder_id}
    detail={detail}
  />
{:else}
  <Dialog.Header>
    <Dialog.Title>Fetching video...</Dialog.Title>
    <Dialog.Description>
      Please wait while we fetch the video details.
    </Dialog.Description>
  </Dialog.Header>
{/if}
