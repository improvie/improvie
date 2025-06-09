<script lang='ts'>
  import type { Content } from '$bindings/item';
  import FloatingTip from '$lib/components/FloatingTip.svelte';
  import IconButton from '$lib/components/IconButton.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn, TimeFormat } from '$lib/utils';
  import { ChevronsLeftIcon, ChevronsRightIcon, PanelBottomOpenIcon, PanelTopOpenIcon, PauseIcon, PlayIcon, RepeatIcon, Volume2Icon, VolumeOffIcon } from '@lucide/svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import TrackExternalContent from './TrackExternalContent.svelte';

  const { track }: { track: Content | undefined } = $props();

  const is_playlist = $derived(tracker.is_playlist());

  let duration = $state(0);

  let sliderCurrentTime = $state(0);

  function to_readable_time(time: number) {
    return TimeFormat.format_secs(TimeFormat.PlainHms, time);
  }

  function onended() {
    if (tracker.next()) {
      sliderCurrentTime = 0;
    }
  }

  $effect(() => {
    if (tracker.paused) {
      return;
    }
    const interval = setInterval(() => {
      sliderCurrentTime = tracker.currentTime;
    }, 1000);
    return () => clearInterval(interval);
  });

  function sliderChange(value: number) {
    if (Math.abs(value - tracker.currentTime) < 1) {
      return;
    }
    tracker.currentTime = value;
  }

  const time = $derived.by(() => {
    return `${to_readable_time(tracker.currentTime)} / ${to_readable_time(duration)}`;
  });

  const thumbnail_path = $derived.by(() => {
    if (!track?.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(track.thumbnail_path);
  });

</script>

<div class={cn('bg-card text-card-foreground sticky z-40 bottom-20 pt-10 pb-5 h-[calc(100dvh-80px)] rounded-none', tracker.external_open || 'hidden')}>
  <TrackExternalContent
    track={track}
    bind:duration
    onended={onended}
  />
</div>

<Card.Root class={cn('sticky bottom-0 h-20 z-40 rounded-none', track || 'hidden')}>
  <Slider class='absolute -translate-y-1/2 left-0' type='single' bind:value={sliderCurrentTime} onValueChange={sliderChange} max={duration} step={1} min={0} />
  <div class='w-full h-full flex justify-between gap-1'>
    <div class='ml-6 gap-2 flex items-center'>
      {#if is_playlist}
        <IconButton onclick={() => {
          tracker.previous();
        }}>
          <ChevronsLeftIcon />
          {#snippet content()}
            <p>previous</p>
          {/snippet}
        </IconButton>
      {/if}
      <IconButton onclick={() => tracker.toggle_pause()}>
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
        <IconButton onclick={() => {
          tracker.next();
        }}>
          <ChevronsRightIcon />
          {#snippet content()}
            <p>next</p>
          {/snippet}
        </IconButton>
      {/if}
      <p class='text-primary text-sm font-mono'>{time}</p>
    </div>
    <div class='gap-2 items-center h-full hidden sm:flex py-4'>
      {#if thumbnail_path}
        <ImageLoader src={thumbnail_path} alt='Thumbnail not found.' />
      {/if}
      <div class='h-full flex items-center'>
        <p class='text-primary text-wrap max-w-[30rem] py-1 line-clamp-3'>{track?.title}</p>
      </div>
    </div>
    <div class='gap-2 flex items-center mr-6'>
      <FloatingTip disableMobile side='left' class='p-4 w-40'>
        {#snippet trigger()}
          {#if tracker.volume === 0}
            <VolumeOffIcon />
          {:else}
            <Volume2Icon />
          {/if}
        {/snippet}

        <Slider type='single' bind:value={tracker.volume} max={1} step={0.01} min={0} />
      </FloatingTip>

      <IconButton variant={tracker.is_looping ? 'secondary' : 'outline'} onclick={() => {
        tracker.toggle_loop();
      }}>
        {#if tracker.is_looping}
          <RepeatIcon />
        {:else}
          <RepeatIcon />
        {/if}
        {#snippet content()}
          {#if tracker.is_looping}
            <p>stop loop</p>
          {:else}
            <p>start loop</p>
          {/if}
        {/snippet}
      </IconButton>

      <IconButton onclick={() => {
        tracker.toggle_external_open();
      }}>
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

    </div>
  </div>
</Card.Root>
