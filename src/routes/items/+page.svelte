<script lang='ts'>
  import IconText from '$lib/components/IconText.svelte';
  import { Button } from '$lib/components/ui/button';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import CreateContentDialog from '$lib/features/dialog/items/CreateContentDialog.svelte';
  import CreateFolderDialog from '$lib/features/dialog/items/CreateFolderDialog.svelte';
  import RenameDialog from '$lib/features/dialog/RenameDialog.svelte';
  import { HierarchyContent, HierarchyFolder } from '$lib/features/hierarchy/items';
  import YtImportDialog from '$lib/features/youtube/YtImportDialog.svelte';
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

{#if yt_open}
  <YtImportDialog bind:open={yt_open} parent_folder_id={current_folder_id} />
{/if}

{#snippet header()}
  <Button
    type='button'
    onclick={() => {
      yt_open = true;
    }}
    variant='ghost'
    size='icon'
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
<ContextMenu.Root>
  <ContextMenu.Trigger>
    <ScrollArea>
      <div class='w-full h-dvh relative'>
        <div class='absolute top-0 pb-50 w-full grid p-4 gap-4 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5'>
          {#each node as child}
            {#if child.kind === 'Folder'}
              <HierarchyFolder folder_id={child.id} bind:rename_data />
            {:else if child.kind === 'Content'}
              <HierarchyContent content_id={child.id} bind:rename_data />
            {/if}
          {/each}
        </div>
      </div>
    </ScrollArea>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => {
      is_open_create_content = true;
    }}>
      <IconText icon={CirclePlusIcon} text='Add Item' />
    </ContextMenu.Item>
    <ContextMenu.Item onclick={() => {
      is_open_create_folder = true;
    }}>
      <IconText icon={FolderIcon} text='Add Folder' />
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
