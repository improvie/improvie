<script lang='ts'>
  import { Button } from '$lib/components/ui/button';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import * as Table from '$lib/components/ui/table/index.js';
  import CreateContentDialog from '$lib/features/dialog/items/CreateContentDialog.svelte';
  import CreateFolderDialog from '$lib/features/dialog/items/CreateFolderDialog.svelte';
  import RenameDialog from '$lib/features/dialog/RenameDialog.svelte';
  import YtImportDialog from '$lib/features/dialog/YtImportDialog.svelte';
  import { HierarchyContent, HierarchyFolder } from '$lib/features/hierarchy/items';
  import { setSlots } from '$lib/stores/index.svelte';
  import { current_folder_ids, folder_nodes } from '$lib/stores/items';
  import { CloudDownloadIcon, FolderIcon, ImportIcon } from 'lucide-svelte';
  import { ItemPageBreadcrumb } from './Breadcrumb.svelte';

  let is_open_create_content = $state(false);
  let is_open_create_folder = $state(false);

  const current_folder_id = $derived($current_folder_ids[$current_folder_ids.length - 1]);
  const node = $derived.by(() => {
    const nodes = $folder_nodes.get(current_folder_id);
    if (!nodes) {
      return [];
    }
    return nodes.items.sort((a, b) => a.sort_order - b.sort_order);
  });

  let rename_data = $state(undefined);

  let yt_open = $state(false);

  setSlots({
    prefix_pathname: '/items',
    header,
  });
</script>

<YtImportDialog bind:open={yt_open} parent_folder_id={current_folder_id} />

{#snippet header()}
  <Button
    type='button'
    onclick={() => {
      yt_open = true;
    }}
    variant='ghost'
    size='icon'
    class='mr-2'
  >
    <CloudDownloadIcon />
  </Button>
{/snippet}

<CreateContentDialog bind:open={is_open_create_content} />
<CreateFolderDialog bind:open={is_open_create_folder} />
<RenameDialog bind:data={rename_data} />

<Separator class='mb-2' />
<ItemPageBreadcrumb />
<Separator class='my-2' />
<ScrollArea class='w-full h-dvh'>
  <Table.Root class='table-fixed select-none'>
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
          <HierarchyFolder folder_id={child.id} bind:rename_data />
        {:else if child.kind === 'Content'}
          <HierarchyContent content_id={child.id} bind:rename_data />
        {/if}
      {/each}
    </Table.Body>
  </Table.Root>
  <ContextMenu.Root>
    <ContextMenu.Trigger class='w-full min-h-72 h-fit'>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={() => {
        is_open_create_content = true;
      }} class='flex items-center'>
        <ImportIcon class='mr-2 size-4' />Add Item
      </ContextMenu.Item>
      <ContextMenu.Item onclick={() => {
        is_open_create_folder = true;
      }} class='flex items-center'>
        <FolderIcon class='mr-2 size-4' />Add Folder
      </ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
</ScrollArea>
