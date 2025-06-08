<script lang='ts'>
  import type { Content } from '$bindings/item';
  import IconButton from '$lib/components/IconButton.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn } from '$lib/utils';
  import { PlayIcon, Volume2Icon } from '@lucide/svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';

  interface Props {
    content: Content;
    idx: number;
  }

  const {
    content,
    idx,
  }: Props = $props();

  const is_current = $derived(tracker.current_rule_idx === idx);

  const thumbnail_path = $derived.by(() => {
    if (!content.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(content.thumbnail_path);
  });

  let is_hovered = $state(false);
</script>

<Card.Root
  onmouseenter={() => is_hovered = true}
  onmouseleave={() => is_hovered = false}
  class={cn('m-6 p-6 relative bg-card', is_current && 'bg-card-primary')}
>
  <div class='absolute left-0 top-0 -translate-x-1/3 -translate-y-1/3'>
    {#if is_current}
      <Volume2Icon size='30' />
    {:else}
      {#if is_hovered}
        <IconButton
          onclick={() => {
            tracker.set_current_track(idx);
          }}
          contentProps={{
            side: 'right',
          }}
        >
          <PlayIcon size='30' />
          {#snippet content()}
            <p>Play this content</p>
          {/snippet}
        </IconButton>
      {/if}
    {/if}
  </div>
  <ImageLoader src={thumbnail_path} />
  <h2 class='line-clamp-2'>{content.title}</h2>
</Card.Root>
