<script module>
  export { default as PlaylistPlayer } from './Player.svelte';
</script>

<script lang='ts'>
  import type { RuleFormat, RuleType } from '$lib/types/rules';
  import { action_get_rules_format } from '$lib/action/rules';

  import { buttonVariants } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { contents } from '$lib/stores/items/content';
  import { cn } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { CirclePlayIcon, CircleStopIcon, ListRestartIcon, RefreshCwIcon } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let { open = $bindable(), rules = $bindable() }: { open: boolean; rules: RuleType[] } = $props();

  let paused = $state(true);
  let is_looping = $state(false);
  let tracks = $state<RuleFormat[]>([]);
  let current_track = $state(0);

  function init_tracks() {
    action_get_rules_format(rules).then((res) => {
      tracks = res;
    });
  }

  function reset() {
    current_track = 0;
    paused = true;
    init_tracks();
  }

  const content = $derived.by(() => {
    if (tracks.length === 0) {
      return undefined;
    }
    return $contents.get(tracks[current_track].content_id);
  });

  onMount(() => {
    init_tracks();
  });

  function onended() {
    if (current_track + 1 < tracks.length) {
      current_track += 1;
      paused = false;
    }
    else {
      current_track = 0;
      if (is_looping) {
        paused = false;
      }
    }
  }
</script>

<Card.Root class={cn('container w-1/3 select-none h-[90dvh] transition-all', open || 'hidden')}>
  <Card.Header>
    <Card.Title>Playlist Player</Card.Title>
  </Card.Header>
  <Card.Content class='h-full'>
    <div class='flex gap-4'>
      <Tooltip.Provider>
        <Tooltip.Root>
          {#if !paused}
            <Tooltip.Trigger class={buttonVariants({ variant: 'destructive', size: 'icon' })} onclick={() => paused = true}>
              <CircleStopIcon />
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>Stop player</p>
            </Tooltip.Content>
          {:else}
            <Tooltip.Trigger class={buttonVariants({ variant: 'outline', size: 'icon' })} onclick={() => paused = false}>
              <CirclePlayIcon />
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>Start player</p>
            </Tooltip.Content>
          {/if}
        </Tooltip.Root>
      </Tooltip.Provider>
      <Tooltip.Provider>
        <Tooltip.Root>
          {#if is_looping}
            <Tooltip.Trigger class={buttonVariants({ variant: 'secondary', size: 'icon' })} onclick={() => is_looping = false}>
              <RefreshCwIcon />
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>Click to Not looping</p>
            </Tooltip.Content>
          {:else}
            <Tooltip.Trigger class={buttonVariants({ variant: 'outline', size: 'icon' })} onclick={() => is_looping = true}>
              <RefreshCwIcon />
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>Click to Looping</p>
            </Tooltip.Content>
          {/if}
        </Tooltip.Root>
      </Tooltip.Provider>
      <Separator orientation='vertical' />
      <Tooltip.Provider>
        <Tooltip.Root>
          <Tooltip.Trigger class={buttonVariants({ variant: 'outline', size: 'icon' })} onclick={reset}>
            <ListRestartIcon />
          </Tooltip.Trigger>
          <Tooltip.Content>
            <p>Start from the beginning</p>
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    </div>
    <h2 class='my-6'>Current Track</h2>
    {#if content === undefined}
      <p>Loading...</p>
    {:else}
      <h3 class='my-4'>{content.title}</h3>
      {#if content.description}
        <p>{content.description}</p>
      {/if}
      <audio controls bind:paused src={convertFileSrc(content.content_path)} onended={onended}>{content.title}</audio>
    {/if}
  </Card.Content>
</Card.Root>
