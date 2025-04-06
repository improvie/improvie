<script lang='ts'>
  import type { Content } from '$lib/types/item';
  import IconButton from '$lib/components/IconButton.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { getLocalStorageOrDefault, setLocalStorage } from '$lib/local-storage';
  import { current_rule_formats, current_rules, current_track_id, set_current_rules } from '$lib/stores/track';
  import { cn, TimeFormat } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { ChevronsLeftIcon, ChevronsRightIcon, PanelBottomOpenIcon, PanelTopOpenIcon, PauseIcon, PlayIcon, RepeatIcon, Volume2Icon, VolumeOffIcon } from 'lucide-svelte';
  import TrackExternalContent from './TrackExternalContent.svelte';

  let { track = $bindable() }: { track: Content } = $props();

  let external_open = $state(false);

  let paused = $state(true);

  let duration = $state(0);

  let currentTime = $state(0);

  let is_looping = $state(getLocalStorageOrDefault('is_looping', 'false') === 'true');

  $effect(() => {
    setLocalStorage('is_looping', is_looping.toString());
  });

  let volume = $state(Number(getLocalStorageOrDefault('volume', '0.5')));
  $effect(() => {
    setLocalStorage('volume', volume.toString());
  });

  let sliderCurrentTime = $state(0);

  function to_readable_time(time: number) {
    return TimeFormat.format_secs(TimeFormat.PlainHms, time);
  }

  function start() {
    paused = false;
    currentTime = 0;
    sliderCurrentTime = 0;
  }

  const is_playlist = $derived($current_rule_formats !== undefined && $current_rules !== undefined);

  function onended() {
    if (is_playlist) {
      if ($current_rule_formats!.idx + 1 < $current_rule_formats!.rules.length) {
        $current_rule_formats!.idx = $current_rule_formats!.idx + 1;
        $current_track_id = $current_rule_formats!.rules[$current_rule_formats!.idx].content_id;
      }
      else {
        if (is_looping) {
          set_current_rules($current_rules!);
        }
      }
    }
    else {
      if (is_looping) {
        start();
      }
    }
  }

  $effect(() => {
    if (paused) {
      return;
    }
    const interval = setInterval(() => {
      sliderCurrentTime = currentTime;
    }, 1000);
    return () => clearInterval(interval);
  });

  function sliderChange(value: number) {
    if (Math.abs(value - currentTime) < 1) {
      return;
    }
    currentTime = value;
  }

  let disable_audio = $state(false);

  const time = $derived.by(() => {
    return `${to_readable_time(currentTime)} / ${to_readable_time(duration)}`;
  });

  const content_path = $derived.by(() => {
    return convertFileSrc(track.content_path);
  });

  const thumbnail_path = $derived.by(() => {
    if (!track.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(track.thumbnail_path);
  });

</script>

<Card.Root class={cn('sticky z-10 bottom-20 pt-10 pb-5 h-[calc(100dvh-80px)]', external_open || 'hidden')}>
  <TrackExternalContent
    bind:content={track}
    bind:paused
    bind:currentTime
    bind:volume
    bind:duration
    bind:disable_audio
    onended={onended}
  />
</Card.Root>

<Card.Root class='sticky bottom-0 h-20 z-20'>
  {#if !disable_audio}
    <audio autoplay bind:volume bind:currentTime bind:paused bind:duration onended={onended} src={content_path}></audio>
  {/if}
  <Slider class='absolute -translate-y-1/2 left-0 mx-2' type='single' bind:value={sliderCurrentTime} onValueChange={sliderChange} max={duration} step={1} min={0} />
  <div class='w-full h-full flex justify-between'>
    <div class='ml-6 gap-4 flex items-center'>
      {#if is_playlist}
        <IconButton onclick={() => {
          if ($current_rule_formats!.idx > 0) {
            $current_rule_formats!.idx = $current_rule_formats!.idx - 1;
            $current_track_id = $current_rule_formats!.rules[$current_rule_formats!.idx].content_id;
          }
        }}>
          <ChevronsLeftIcon />
          {#snippet content()}
            <p>previous</p>
          {/snippet}
        </IconButton>
      {/if}
      <IconButton onclick={() => paused = !paused}>
        {#if paused}
          <PlayIcon />
        {:else}
          <PauseIcon />
        {/if}
        {#snippet content()}
          {#if paused}
            <p>start content</p>
          {:else}
            <p>pause content</p>
          {/if}
        {/snippet}
      </IconButton>
      {#if is_playlist}
        <IconButton onclick={() => {
          if ($current_rule_formats!.idx + 1 < $current_rule_formats!.rules.length) {
            $current_rule_formats!.idx = $current_rule_formats!.idx + 1;
            $current_track_id = $current_rule_formats!.rules[$current_rule_formats!.idx].content_id;
          }
        }}>
          <ChevronsRightIcon />
          {#snippet content()}
            <p>next</p>
          {/snippet}
        </IconButton>
      {/if}
      <p class='text-primary text-sm font-mono'>{time}</p>
    </div>
    <div class='gap-4 items-center h-full flex py-4'>
      {#if thumbnail_path}
        <img class='h-full w-auto aspect-video object-cover' src={thumbnail_path} alt='Thumbnail not found.' />
      {/if}
      <div class='h-full flex items-center'>
        <p class='text-primary text-wrap max-w-[30rem]'>{track.title}</p>
      </div>
    </div>
    <div class='gap-4 flex items-center mr-6'>
      <Tooltip.Root>
        <Tooltip.Trigger>
          {#if volume === 0}
            <VolumeOffIcon />
          {:else}
            <Volume2Icon />
          {/if}
        </Tooltip.Trigger>
        <Tooltip.Content side='left' class='p-4 w-40'>
          <Slider type='single' bind:value={volume} max={1} step={0.01} min={0} />
        </Tooltip.Content>
      </Tooltip.Root>

      <IconButton variant={is_looping ? 'secondary' : 'outline'} onclick={() => {
        is_looping = !is_looping;
      }}>
        {#if is_looping}
          <RepeatIcon />
        {:else}
          <RepeatIcon />
        {/if}
        {#snippet content()}
          {#if is_looping}
            <p>stop loop</p>
          {:else}
            <p>start loop</p>
          {/if}
        {/snippet}
      </IconButton>

      <IconButton onclick={() => {
        external_open = !external_open;
      }}>
        {#if external_open}
          <PanelTopOpenIcon />
        {:else}
          <PanelBottomOpenIcon />
        {/if}
        {#snippet content()}
          {#if external_open}
            <p>close</p>
          {:else}
            <p>open</p>
          {/if}
        {/snippet}
      </IconButton>

    </div>
  </div>
</Card.Root>
