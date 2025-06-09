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
    disableMobile = false,
    children,
    trigger,
  }: {
    side?: Side;
    class?: string;
    disableMobile?: boolean;
    children?: Snippet;
    trigger?: Snippet;
  } = $props();
</script>

{#if mobile.current}
  {#if !disableMobile}
    <Popover.Root>
      <Popover.Trigger>
        {@render trigger?.()}
      </Popover.Trigger>
      <Popover.Content side={side} class={className}>
        {@render children?.()}
      </Popover.Content>
    </Popover.Root>
  {/if}
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
