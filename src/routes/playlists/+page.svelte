<script lang='ts'>
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import * as Table from '$lib/components/ui/table/index.js';
  import CreatePlayFolderDialog from '$lib/features/dialog/plays/CreatePlayFolderDialog.svelte';
  import CreatePlaylistDialog from '$lib/features/dialog/plays/CreatePlaylistDialog.svelte';
  import { HierarchyPlayerFolder, HierarchyPlaylist } from '$lib/features/hierarchy/plays';
  import { current_play_folder_ids, init_play_items, play_folder_nodes } from '$lib/stores/plays';
  import { FolderIcon, ImportIcon } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { PlayPageBreadcrumb } from './Breadcrumb.svelte';

  onMount(() => {
    init_play_items();
  });

  let is_open_create_playlist = $state(false);
  let is_open_create_play_folder = $state(false);

  const current_play_folder_id = $derived($current_play_folder_ids[$current_play_folder_ids.length - 1]);
  const node = $derived.by(() => {
    const nodes = $play_folder_nodes.get(current_play_folder_id);
    if (!nodes) {
      return [];
    }
    return nodes.children.sort((a, b) => a.sort_order - b.sort_order);
  });
</script>

<CreatePlaylistDialog bind:open={is_open_create_playlist} />
<CreatePlayFolderDialog bind:open={is_open_create_play_folder} />

<Separator class='my-2' />
<PlayPageBreadcrumb />
<Separator class='my-2' />
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
        <HierarchyPlayerFolder folder_id={child.id} />
      {:else if child.kind === 'Playlist'}
        <HierarchyPlaylist playlist_id={child.id} />
      {/if}
    {/each}
  </Table.Body>
</Table.Root>
<ContextMenu.Root>
  <ContextMenu.Trigger class='w-full h-[-webkit-fill-available]'>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => {
      is_open_create_playlist = true;
    }} class='flex items-center'>
      <ImportIcon class='mr-2 size-4' />Add Playlsit
    </ContextMenu.Item>
    <ContextMenu.Item onclick={() => {
      is_open_create_play_folder = true;
    }} class='flex items-center'>
      <FolderIcon class='mr-2 size-4' />Add Folder
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
