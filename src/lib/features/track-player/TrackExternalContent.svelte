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

  let video_element: HTMLVideoElement | undefined = $state();

  $effect(() => {
    if (video_element) {
      if (content_path) {
        video_element.load();
        video_element.play().catch((error) => {
          Logger.error(`Error playing video: ${error}`);
        });
      }
      else {
        video_element.pause();
      }
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
  <Tabs.Content value='thumbnail' class={cn('pt-2 h-full flex items-center justify-center', value !== 'thumbnail' && 'hidden')}>
    <ImageLoader
      src={thumbnail_path}
    />
  </Tabs.Content>
  <Tabs.Content value='video' class={cn('pt-2 h-full flex items-center justify-center', value !== 'video' && 'hidden')}>
    <video
      bind:this={video_element}
      crossorigin='anonymous'
      playsinline
      bind:volume={tracker.volume}
      bind:currentTime={tracker.currentTime}
      bind:paused={tracker.paused}
      bind:duration
      onended={onended}
      class='aspect-video w-full h-fit object-contain'
      onclick={() => tracker.toggle_pause()}
    >
      {#if content_path}
        <source src={content_path} />
      {/if}
      <track kind='captions' />
    </video>
  </Tabs.Content>
</Tabs.Root>
