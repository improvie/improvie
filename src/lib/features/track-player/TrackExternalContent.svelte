<script lang='ts'>
  import type { Content } from '$bindings/item';
  import IconButton from '$lib/components/IconButton.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { Logger } from '$lib/logger';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn, TimeFormat } from '$lib/utils';
  import { ChevronsLeftIcon, ChevronsRightIcon, PanelBottomOpenIcon, PanelTopOpenIcon, PauseIcon, PlayIcon, Repeat1Icon, RepeatIcon } from '@lucide/svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { toast } from 'svelte-sonner';

  let {
    track,
    duration = $bindable(),
    onended,
    sliderChangeStart,
  }: {
    track: Content | undefined;
    duration: number;
    onended: () => void;
    sliderChangeStart: (value: number) => void;
  } = $props();

  const is_video = $derived(track?.kind === 'Video');
  const is_playlist = $derived(tracker.is_playlist());

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

  function ontimeupdate() {
    tracker.currentTime = video_element.currentTime;
  }

  $effect(() => {
    if (Math.abs(tracker.currentTime - video_element.currentTime) > 1) {
      video_element.currentTime = tracker.currentTime;
    }
  });

  function notFound() {
    toast.error('Failed load video. (maybe video file deleted)', {
      duration: 5000,
    });
  }
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
          ontimeupdate={() => ontimeupdate()}
          bind:paused={tracker.paused}
          bind:duration
          onended={onended}
          class='aspect-video object-contain w-full h-full'
          onclick={() => tracker.toggle_pause()}
          poster={thumbnail_path}
          onerror={() => notFound()}
        >
          <source src={content_path} onerror={() => notFound()} />
          <track kind='captions' />
        </video>
      </div>
    </div>

    <div class='w-full flex flex-col sm:hidden absolute bottom-0 p-6 gap-4'>
      <div class='w-full flex flex-col gap-1'>
        <Slider
          type='single'
          bind:value={tracker.currentTime}
          onValueChangeStart={sliderChangeStart}
          max={duration}
          min={0}
        />
        <div class='flex justify-between text-xs text-muted-foreground'>
          <span>{TimeFormat.format_secs(TimeFormat.PlainHms, tracker.currentTime)}</span>
          <span>{TimeFormat.format_secs(TimeFormat.PlainHms, duration)}</span>
        </div>
      </div>
      <div class='flex items-center justify-between px-8'>
        <IconButton
          class='scale-110'
          onclick={() => { tracker.toggle_external_open(); }}
        >
          {#if tracker.external_open}
            <PanelTopOpenIcon />
          {:else}
            <PanelBottomOpenIcon />
          {/if}
          {#snippet content()}
            {#if tracker.external_open}
              <p>close</p>
            {:else}
              <p>open</p>
            {/if}
          {/snippet}
        </IconButton>
        {#if is_playlist}
          <IconButton
            class='scale-110'
            onclick={() => { tracker.previous(); }}
          >
            <ChevronsLeftIcon />
            {#snippet content()}
              <p>previous</p>
            {/snippet}
          </IconButton>
        {/if}
        <IconButton
          onclick={() => tracker.toggle_pause()}
          class='scale-160'
        >
          {#if tracker.paused}
            <PlayIcon />
          {:else}
            <PauseIcon />
          {/if}
          {#snippet content()}
            {#if tracker.paused}
              <p>start content</p>
            {:else}
              <p>pause content</p>
            {/if}
          {/snippet}
        </IconButton>
        {#if is_playlist}
          <IconButton
            class='scale-110'
            onclick={() => { tracker.next(); }}
          >
            <ChevronsRightIcon />
            {#snippet content()}
              <p>next</p>
            {/snippet}
          </IconButton>
        {/if}
        <IconButton
          class='scale-110'
          pressed={tracker.loop_state !== 'off'}
          onclick={() => { tracker.toggle_loop(); }}
        >
          {#if tracker.loop_state === 'single'}
            <Repeat1Icon />
          {:else}
            <RepeatIcon />
          {/if}
          {#snippet content()}
            {#if tracker.loop_state === 'full'}
              <p>stop loop</p>
            {:else if tracker.loop_state === 'single'}
              <p>start full loop</p>
            {:else}
              <p>start single loop</p>
            {/if}
          {/snippet}
        </IconButton>
      </div>
    </div>
  </div>
</Tabs.Root>
