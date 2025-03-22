<script lang='ts'>
  import type { Content } from '$lib/types/item';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';

  let { content = $bindable() }: { content: Content | undefined } = $props();

</script>

{#if content}
  <ContextMenu.Root>
    <ContextMenu.Content>
      <ContextMenu.Item>Profile</ContextMenu.Item>
      <ContextMenu.Item>Billing</ContextMenu.Item>
      <ContextMenu.Item>Team</ContextMenu.Item>
      <ContextMenu.Item>Subscription</ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
