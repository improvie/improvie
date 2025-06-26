<script lang='ts'>
  import type { Content } from '$bindings/item';
  import IconButton from '$lib/components/IconButton.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn, TimeFormat } from '$lib/utils';
  import { ChevronsLeftIcon, ChevronsRightIcon, PanelBottomOpenIcon, PanelTopOpenIcon, PauseIcon, PlayIcon, RepeatIcon, Volume2Icon, VolumeOffIcon } from '@lucide/svelte';
  import TrackExternalContent from './TrackExternalContent.svelte';

  const { track }: { track: Content | undefined } = $props();

  const is_running = $derived(track !== undefined);

  const is_playlist = $derived(tracker.is_playlist());

  let duration = $state(0);

  let sliderCurrentTime = $state(0);

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

</script>

<div id='external' class={cn(
  'bg-card text-card-foreground sticky z-40 bottom-20 pt-10 pb-5 h-[calc(100dvh-80px)] rounded-none',
  (tracker.external_open && 'custom-show') || 'custom-hidden',
  (is_running && 'custom-running') || 'custom-stopped',
)}>
  <TrackExternalContent
    track={track}
    bind:duration
    onended={onended}
    bind:sliderCurrentTime
    sliderChange={sliderChange}
  />
</div>

<style>
  #external.custom-stopped {
    visibility: hidden;
  }
  #external.custom-show {
    animation: slide-up 0.3s ease-out forwards;
  }
  #external.custom-hidden {
    animation: slide-down 0.3s ease-in forwards;
  }

  @keyframes slide-up {
    from {
      opacity: 0;
      transform: translateY(100%);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes slide-down {
    from {
      opacity: 1;
      transform: translateY(0);
    }
    to {
      opacity: 0;
      display: none;
      transform: translateY(100%);
    }
  }
</style>

<div
  class={cn('group bg-card py-3 sticky bottom-0 h-20 z-40', track || 'hidden')}
  data-hidden={tracker.external_open}
>
  <Slider
    class='absolute top-0 max-sm:group-data-[hidden=true]:hidden'
    type='single'
    bind:value={sliderCurrentTime}
    onValueChange={sliderChange}
    max={duration}
    step={1}
    min={0}
  />
  <div class='px-6 w-full h-full flex justify-between gap-1 flex-row-reverse sm:flex-row' role='button' tabindex='-1' onclick={(event) => {
    if (event.target === event.currentTarget) {
      tracker.external_open = !tracker.external_open;
    }
  }} onkeydown={() => {}}>
    <div class='gap-2 flex items-center'>
      <div class='hidden sm:block'>
        {#if is_playlist}
          <IconButton onclick={() => { tracker.previous(); }}>
            <ChevronsLeftIcon />
            {#snippet content()}
              <p>previous</p>
            {/snippet}
          </IconButton>
        {/if}
      </div>
      <IconButton
        onclick={() => tracker.toggle_pause()}
        class='max-sm:group-data-[hidden=true]:hidden'
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
          class='max-sm:group-data-[hidden=true]:hidden'
          onclick={() => { tracker.next(); }}
        >
          <ChevronsRightIcon />
          {#snippet content()}
            <p>next</p>
          {/snippet}
        </IconButton>
      {/if}
      <div class='flex-col items-center text-primary text-sm font-mono hidden sm:flex'>
        <p>
          {TimeFormat.format_secs(TimeFormat.PlainHms, tracker.currentTime)}
        </p>
        <Separator class='!h-[3px] bg-muted-foreground' />
        <p>
          {TimeFormat.format_secs(TimeFormat.PlainHms, duration)}
        </p>
      </div>
    </div>
    <div class='gap-2 flex items-center h-full' role='button' tabindex='-1' onclick={() => {
      tracker.external_open = !tracker.external_open;
    }} onkeydown={() => {}}>
      <ImageLoader direction='vertical' local src={track?.thumbnail_path} class='sm:max-md:hidden' />
      <div class='h-full flex items-center'>
        <p class='text-primary text-sm text-wrap max-w-[30rem] line-clamp-3'>{track?.title}</p>
      </div>
    </div>
    <div class='gap-2 items-center hidden sm:flex'>
      <Tooltip.Root>
        <Tooltip.Trigger class='hidden md:block'>
          {#if tracker.volume === 0}
            <VolumeOffIcon />
          {:else}
            <Volume2Icon />
          {/if}
        </Tooltip.Trigger>
        <Tooltip.Content side='left' class='p-4 w-40 bg-secondary' arrowClasses='bg-secondary'>
          <Slider type='single' bind:value={tracker.volume} max={1} step={0.01} min={0} />
        </Tooltip.Content>
      </Tooltip.Root>

      <IconButton pressed={tracker.is_looping} onclick={() => {
        tracker.toggle_loop();
      }}>
        <RepeatIcon />
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
</div>
