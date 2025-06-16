<script lang='ts'>
  import type { Content } from '$bindings/item';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { Logger } from '$lib/logger';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn, TimeFormat } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let {
    track,
    duration = $bindable(),
    onended,
    sliderCurrentTime = $bindable(),
    sliderChange,
  }: {
    track: Content | undefined;
    duration: number;
    onended: () => void;
    sliderCurrentTime: number;
    sliderChange: (value: number) => void;
  } = $props();

  const is_video = $derived(track?.kind === 'Video');

  let value: string = $state(track?.kind === 'Video' ? 'video' : 'thumbnail');

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
      tracker.paused = true;
      video_element.play().catch((error) => {
        Logger.error(`Error playing video: ${error}`);
      });
      tracker.paused = false;
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
  <div class='pt-2 h-full flex flex-col relative'>
    <div class='w-full h-fit aspect-video absolute top-[50%] left-[50%] -translate-x-1/2 -translate-y-1/2'>
      <div class={cn(value !== 'thumbnail' && 'hidden')}>
        <ImageLoader
          src={thumbnail_path}
        />
      </div>
      <div class={cn(value !== 'video' && 'hidden')}>
        <video
          bind:this={video_element}
          crossorigin='anonymous'
          playsinline
          bind:volume={tracker.volume}
          bind:currentTime={tracker.currentTime}
          bind:paused={tracker.paused}
          bind:duration
          onended={onended}
          class='object-cover'
          onclick={() => tracker.toggle_pause()}
          poster={thumbnail_path}
        >
          <source src={content_path} />
          <track kind='captions' />
        </video>
      </div>
    </div>

    <div class='w-full flex sm:hidden absolute bottom-0 p-6'>
      <div class='w-full flex flex-col gap-1'>
        <Slider
          type='single'
          bind:value={sliderCurrentTime}
          onValueChange={sliderChange}
          max={duration}
          step={1}
          min={0}
        />
        <div class='flex justify-between text-xs text-muted-foreground'>
          <span>{TimeFormat.format_secs(TimeFormat.PlainHms, sliderCurrentTime)}</span>
          <span>{TimeFormat.format_secs(TimeFormat.PlainHms, duration)}</span>
        </div>
      </div>
    </div>
  </div>
</Tabs.Root>
