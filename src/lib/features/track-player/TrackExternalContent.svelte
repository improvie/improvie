<script lang='ts'>
  import type { Content } from '$bindings/item';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let {
    track,
    duration = $bindable(),
    disable_audio = $bindable(),
    onended,
  }: {
    track: Content;
    duration: number;
    disable_audio: boolean;
    onended: () => void;
  } = $props();

  const is_video = $derived(track.kind === 'Video');

  let value: string = $state('thumbnail');

  $effect(() => {
    if (is_video) {
      disable_audio = true;
      value = 'video';
    }
    else {
      value = 'thumbnail';
    }
  });

  const content_path = $derived.by(() => {
    return convertFileSrc(track.content_path);
  });

  let thumbnail_path = $derived.by(() => {
    if (!track.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(track.thumbnail_path);
  });

  let video_element: HTMLVideoElement | undefined = $state();

  $effect(() => {
    if (video_element && content_path) {
      video_element.load();
      video_element.play();
    }
  });
</script>

<Tabs.Root bind:value class='container mx-auto text-center h-full'>
  <Tabs.List class='absolute top-2 -translate-x-1/2'>
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
      bind:src={thumbnail_path}
    />
  </Tabs.Content>
  <Tabs.Content value='video' class={cn('pt-2 h-full flex items-center justify-center', value !== 'video' && 'hidden')}>
    {#if disable_audio}
      <video bind:this={video_element} crossorigin='anonymous' playsinline autoplay bind:volume={tracker.volume} bind:currentTime={tracker.currentTime} bind:paused={tracker.paused} bind:duration onended={onended} class='h-full w-auto object-contain' onclick={() => tracker.toggle_pause()}>
        <source src={content_path} />
        <track kind='captions' />
      </video>
    {/if}
  </Tabs.Content>
</Tabs.Root>
