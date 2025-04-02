<script lang='ts'>
  import type { Content } from '$lib/types/item';
  import { Button } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Slider } from '$lib/components/ui/slider/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { getLocalStorageDefault, setLocalStorage } from '$lib/local-storage';
  import { TimeFormat } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { PauseIcon, PlayIcon, RepeatIcon, Volume2Icon, VolumeOffIcon } from 'lucide-svelte';

  let { track = $bindable() }: { track: Content } = $props();

  $inspect(track);

  let paused = $state(true);

  let duration = $state(0);

  let currentTime = $state(0);

  let is_looping = $state(false);

  let volume = $state(Number(getLocalStorageDefault('volume', '0.5')));
  $effect(() => {
    setLocalStorage('volume', volume.toString());
  });

  let sliderCurrentTime = $state(0);

  function to_readable_time(time: number) {
    return TimeFormat.format_secs(TimeFormat.PlainHms, time);
  }

  function onended() {
    if (is_looping) {
      paused = false;
      currentTime = 0;
      sliderCurrentTime = 0;
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
</script>

<Card.Root class='sticky bottom-0 h-20'>
  <audio bind:volume bind:currentTime bind:paused bind:duration onended={onended} src={convertFileSrc(track.content_path)}></audio>
  <Slider class='absolute -translate-y-1/2 mx-2' type='single' bind:value={sliderCurrentTime} onValueChange={sliderChange} max={duration} step={1} min={0} />
  <div class='w-full h-full flex justify-between'>
    <div class='ml-6 gap-4 flex items-center'>
      <Button variant='outline' size='icon' onclick={() => paused = !paused}>
        {#if paused}
          <PlayIcon />
        {:else}
          <PauseIcon />
        {/if}
      </Button>
      <p class='text-primary text-sm'>{to_readable_time(currentTime)} / {to_readable_time(duration)}</p>
    </div>
    <div class='gap-4 items-center h-full flex py-4'>
      {#if track.thumbnail_path}
        <div class='h-full aspect-square relative flex items-center'>
          <img alt={track.title} class='object-cover' src={convertFileSrc(track.thumbnail_path)} />
        </div>
      {/if}
      <div class='h-full'>
        <p class='text-primary text-wrap max-w-80'>{track.title}</p>
      </div>
    </div>
    <div class='gap-4 flex items-center mr-6'>
      <Tooltip.Provider>
        <Tooltip.Root delayDuration={0}>
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
      </Tooltip.Provider>

      {#if is_looping}
        <Button variant='secondary' size='icon' onclick={() => {
          is_looping = !is_looping;
        }}>
          <RepeatIcon />
        </Button>
      {:else}
        <Button variant='outline' size='icon' onclick={() => {
          is_looping = !is_looping;
        }}>
          <RepeatIcon />
        </Button>
      {/if}

    </div>
  </div>
</Card.Root>
