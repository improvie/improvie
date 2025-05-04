<script lang='ts'>
  import { Button } from '$lib/components/ui/button';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import CreateContentDialog from '$lib/features/dialog/items/CreateContentDialog.svelte';
  import CreateFolderDialog from '$lib/features/dialog/items/CreateFolderDialog.svelte';
  import RenameDialog from '$lib/features/dialog/RenameDialog.svelte';
  import YtImportDialog from '$lib/features/dialog/YtImportDialog.svelte';
  import { HierarchyContent, HierarchyFolder } from '$lib/features/hierarchy/items';
  import { setSlots } from '$lib/stores/index.svelte';
  import { current_folder_ids, folder_nodes } from '$lib/stores/items';
  import { CirclePlusIcon, CloudDownloadIcon, FolderIcon } from '@lucide/svelte';
  import { ItemPageBreadcrumb } from './Breadcrumb.svelte';

  let is_open_create_content = $state(false);
  let is_open_create_folder = $state(false);

  const current_folder_id = $derived($current_folder_ids[$current_folder_ids.length - 1]);
  const node = $derived.by(() => {
    const nodes = folder_nodes.get(current_folder_id);
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
    class='mr-1'
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
<div class='w-full h-dvh relative overflow-y-auto'>
  <ContextMenu.Root>
    <ContextMenu.Trigger class='absolute w-full h-full z-10'>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={() => {
        is_open_create_content = true;
      }}>
        <CirclePlusIcon class='mr-1 size-4' />Add Item
      </ContextMenu.Item>
      <ContextMenu.Item onclick={() => {
        is_open_create_folder = true;
      }}>
        <FolderIcon class='mr-1 size-4' />Add Folder
      </ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>

  <div class='absolute grid p-4 gap-4 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5'>
    {#each node as child}
      {#if child.kind === 'Folder'}
        <HierarchyFolder folder_id={child.id} bind:rename_data />
      {:else if child.kind === 'Content'}
        <HierarchyContent content_id={child.id} bind:rename_data />
      {/if}
    {/each}
  </div>
</div>
