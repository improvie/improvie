<script lang='ts'>
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import * as Table from '$lib/components/ui/table/index.js';
  import CreateFolderDialog from '$lib/features/dialog/items/CreateFolderDialog.svelte';
  import CreateItemDialog from '$lib/features/dialog/items/CreateItemDialog.svelte';
  import { HierarchyContent, HierarchyFolder } from '$lib/features/hierarchy/items';
  import { current_folder_ids, folder_nodes } from '$lib/stores/items';
  import { FolderIcon, ImportIcon } from 'lucide-svelte';
  import { ItemPageBreadcrumb } from './Breadcrumb.svelte';

  let content_open = $state(false);
  let folder_open = $state(false);

  const current_folder_id = $derived($current_folder_ids[$current_folder_ids.length - 1]);
  const node = $derived.by(() => {
    const nodes = $folder_nodes.get(current_folder_id);
    if (!nodes) {
      return [];
    }
    return nodes.items.sort((a, b) => a.sort_order - b.sort_order);
  });
</script>

<CreateItemDialog bind:open={content_open} />
<CreateFolderDialog bind:open={folder_open} />

<Separator class='my-2' />
<ItemPageBreadcrumb />
<Separator class='my-2' />
<ContextMenu.Root>
  <ContextMenu.Trigger class='h-full w-full'>
    <Table.Root class='table-fixed'>
      <Table.Header>
        <Table.Row>
          <Table.Head class='w-8'>Name</Table.Head>
          <Table.Head></Table.Head>
          <!-- <Table.Head>Description</Table.Head> -->
          <Table.Head class='text-right'>CreatedAt</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each node as child}
          {#if child.kind === 'Folder'}
            <HierarchyFolder folder_id={child.id} />
          {:else if child.kind === 'Content'}
            <HierarchyContent content_id={child.id} />
          {/if}
        {:else}
          <p>Empty</p>
        {/each}
      </Table.Body>
    </Table.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => {
      content_open = true;
    }} class='flex items-center'>
      <ImportIcon class='mr-2 size-4' />Add Item
    </ContextMenu.Item>
    <ContextMenu.Item onclick={() => {
      folder_open = true;
    }} class='flex items-center'>
      <FolderIcon class='mr-2 size-4' />Add Folder
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
