<script module>

  export { default as PlaylistPlayer } from './Player.svelte';
</script>

<script lang='ts'>
  import type { CurrentRule } from '$lib/stores/track';
  import * as Card from '$lib/components/ui/card/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
  import { contents } from '$lib/stores/items/content';
  import { current_rule_formats } from '$lib/stores/track';
  import { cn } from '$lib/utils';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { ImageOffIcon } from 'lucide-svelte';

  let { open = $bindable() }: { open: boolean } = $props();

  const current: CurrentRule = $derived.by(() => {
    if ($current_rule_formats === undefined) {
      return {
        rules: [],
        idx: 0,
      };
    }
    return $current_rule_formats;
  });

</script>

<Card.Root class={cn('container w-1/3 select-none h-[90dvh] transition-all py-6 flex flex-col', open || 'hidden')}>
  <ScrollArea class='w-full max-h-fit'>
    {#each current.rules as rule, _}
      {@const content = $contents.get(rule.content_id)}
      {#if content !== undefined}
        <Card.Root class='min-w-80'>
          <Card.Content>
            <div class='w-full'>
              {#if content.thumbnail_path}
                <img
                  src={convertFileSrc(content.thumbnail_path)}
                  alt='Thumbnail not found.'
                  class='w-full h-auto object-contain'
                />
              {:else}
                <ImageOffIcon class='w-80 h-80' />
              {/if}
            </div>
            <h2>{content.title}</h2>
          </Card.Content>
        </Card.Root>
      {/if}
    {/each}
  </ScrollArea>
</Card.Root>
