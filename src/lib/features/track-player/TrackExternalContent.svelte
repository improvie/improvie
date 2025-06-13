<script lang='ts'>
  import type { Content } from '$bindings/item';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { Logger } from '$lib/logger';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let {
    track,
    duration = $bindable(),
    onended,
  }: {
    track: Content | undefined;
    duration: number;
    onended: () => void;
  } = $props();

  const is_video = $derived(track?.kind === 'Video');

  let value: string = $derived(is_video ? 'video' : 'thumbnail');

  const content_path = $derived.by(() => {
    if (!track?.content_path) {
      return undefined;
    }
    return convertFileSrc(track.content_path);
  });

  const thumbnail_path = $derived.by(() => {
    if (!track?.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(track.thumbnail_path);
  });
  let video_element: HTMLVideoElement;

  $effect(() => {
    if (content_path) {
      video_element.load();
      video_element.play().catch((error) => {
        Logger.error(`Error playing video: ${error}`);
      });
    }
    else {
      video_element.pause();
    }
  });
</script>

<Tabs.Root bind:value class='container mx-auto text-center h-full'>
  <Tabs.List class='absolute top-2 left-1/2 -translate-x-1/2 flex items-center justify-center'>
    <Tabs.Trigger value='thumbnail'>Thumbnail</Tabs.Trigger>
    {#if is_video}
      <Tabs.Trigger value='video'>Video</Tabs.Trigger>
    {:else}
      <Tooltip.Root>
        <Tooltip.Trigger class='cursor-not-allowed'>
          <Tabs.Trigger value='video' disabled>Video</Tabs.Trigger>
        </Tooltip.Trigger>
        <Tooltip.Content>
          <p>This content is not Video</p>
        </Tooltip.Content>
      </Tooltip.Root>
    {/if}
  </Tabs.List>
  <div class='pt-2 h-full flex flex-col items-center justify-center'>
    <ImageLoader
      src={thumbnail_path}
      class={cn(value !== 'thumbnail' && 'hidden')}
    />

    <video
      bind:this={video_element}
      crossorigin='anonymous'
      playsinline
      bind:volume={tracker.volume}
      bind:currentTime={tracker.currentTime}
      bind:paused={tracker.paused}
      bind:duration
      onended={onended}
      class={cn('aspect-video w-full h-fit object-contain', value !== 'video' && 'hidden')}
      onclick={() => tracker.toggle_pause()}
      poster={thumbnail_path}
    >
      <source src={content_path} />
      <track kind='captions' />
    </video>
    <div class='flex sm:hidden'>
      <p>Impl the menu for mobile</p>
      <!-- TODO: Implement the menu for mobile -->
    </div>
  </div>
</Tabs.Root>
