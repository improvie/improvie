<script module>
  export { default as EditorTracker } from './Tracker.svelte';
</script>

<script lang='ts'>
  import * as Card from '$lib/components/ui/card/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
  import { contents } from '$lib/stores/items/content';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn } from '$lib/utils';
  import TrackerContent from './TrackerContent.svelte';

  let {
    open = $bindable(),
  }: {
    open: boolean;
  } = $props();

</script>

<Card.Root class={cn('p-0 container w-1/3 select-none h-[90dvh]', open || 'hidden')}>
  <ScrollArea class='w-full h-full'>
    {#each tracker.play_rules as rule, idx}
      {@const content = contents.get(rule.content_id)}
      {#if content !== undefined}
        <TrackerContent content={content} idx={idx} />
      {/if}
    {/each}
  </ScrollArea>
</Card.Root>
