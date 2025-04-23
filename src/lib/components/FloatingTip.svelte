<script lang='ts'>
  import type { Snippet } from 'svelte';
  import type { Side } from './ui/sheet/sheet-content.svelte';
  import * as Popover from '$lib/components/ui/popover/index.js';
  import * as Tooltip from '$lib/components/ui/tooltip/index.js';
  import { IsMobile } from '$lib/hooks/is-mobile.svelte';

  const mobile = new IsMobile();

  const {
    side,
    class: className,
    children,
    trigger,
  }: {
    side?: Side;
    class?: string;
    children?: Snippet;
    trigger?: Snippet;
  } = $props();
</script>

{#if mobile.current}
  <Popover.Root>
    <Popover.Trigger>
      {@render trigger?.()}
    </Popover.Trigger>
    <Popover.Content side={side} class={className}>
      {@render children?.()}
    </Popover.Content>
  </Popover.Root>
{:else}
  <Tooltip.Root>
    <Tooltip.Trigger>
      {@render trigger?.()}
    </Tooltip.Trigger>
    <Tooltip.Content side={side} class={className}>
      {@render children?.()}
    </Tooltip.Content>
  </Tooltip.Root>
{/if}
