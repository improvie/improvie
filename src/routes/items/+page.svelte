<script lang='ts'>
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { HierarchyContent, HierarchyFolder } from '$lib/features/hierarchy/items';
  import { current_folder_ids, folder_nodes } from '$lib/stores/items';
  import { ItemPageBreadcrumb } from './Breadcrumb.svelte';
  import { ItemPageButtons } from './Buttons.svelte';

  const current_folder_id = $derived($current_folder_ids[$current_folder_ids.length - 1]);
  const node = $derived.by(() => {
    const nodes = $folder_nodes.get(current_folder_id);
    if (!nodes) {
      return [];
    }
    return nodes.items.sort((a, b) => a.sort_order - b.sort_order);
  });
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger class='h-full w-full'>
    <ItemPageButtons />
    <Separator class='my-2' />
    <ItemPageBreadcrumb />
    <Separator class='my-2' />
    <div class='px-2'>
      {#each node as child}
        {#if child.kind === 'Folder'}
          <HierarchyFolder folder_id={child.id} />
        {:else if child.kind === 'Content'}
          <HierarchyContent content_id={child.id} />
        {/if}
      {:else}
        <p>Empty</p>
      {/each}
    </div>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item>Profile</ContextMenu.Item>
    <ContextMenu.Item>Billing</ContextMenu.Item>
    <ContextMenu.Item>Team</ContextMenu.Item>
    <ContextMenu.Item>Subscription</ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
