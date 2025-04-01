<script module>
  export { default as PlaylistPlayer } from './Player.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$lib/types/plays';
  import type { RuleFormat, RuleType } from '$lib/types/rules';
  import { action_get_rules_format } from '$lib/action/rules';

  import { buttonVariants } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { contents } from '$lib/stores/items/content';
  import { cn } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { CirclePlayIcon, CircleStopIcon, ListRestartIcon } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let { playlist = $bindable(), open = $bindable(), rules = $bindable() }: { playlist: Playlist; open: boolean; rules: RuleType[] } = $props();

  let is_playing = $state(false);
  let tracks = $state<RuleFormat[]>([]);
  let current_track = $state(0);

  function init_tracks() {
    action_get_rules_format(rules).then((res) => {
      tracks = res;
    });
  }

  function reset() {
    current_track = 0;
    is_playing = false;
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
</script>

<Card.Root class={cn('container w-1/3 select-none h-[90dvh] transition-all', open || 'hidden')}>
  <Card.Header>
    <Card.Title>Playlist Player</Card.Title>
  </Card.Header>
  <Card.Content class='h-full'>
    <div class='flex gap-4'>
      <Tooltip.Provider>
        <Tooltip.Root>
          {#if is_playing}
            <Tooltip.Trigger class={buttonVariants({ variant: 'destructive', size: 'icon' })} onclick={() => is_playing = false}>
              <CircleStopIcon />
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>Stop player</p>
            </Tooltip.Content>
          {:else}
            <Tooltip.Trigger class={buttonVariants({ variant: 'secondary', size: 'icon' })} onclick={() => is_playing = true}>
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
          <Tooltip.Trigger class={buttonVariants({ variant: 'outline', size: 'icon' })} onclick={reset}>
            <ListRestartIcon />
          </Tooltip.Trigger>
          <Tooltip.Content>
            <p>Start from the beginning</p>
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    </div>
    {#if content === undefined}
      <p>Loading...</p>
    {:else}
      <h2>{content.title}</h2>
      {#if content.description}
        <p>{content.description}</p>
      {/if}
      <audio controls src={convertFileSrc(content.content_path)}>{content.title}</audio>
    {/if}
  </Card.Content>
</Card.Root>
