<script lang='ts'>
  import IconText from '$lib/components/IconText.svelte';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import CreatePlayFolderDialog from '$lib/features/dialog/plays/CreatePlayFolderDialog.svelte';
  import CreatePlaylistDialog from '$lib/features/dialog/plays/CreatePlaylistDialog.svelte';
  import RenameDialog from '$lib/features/dialog/RenameDialog.svelte';
  import { HierarchyPlayerFolder, HierarchyPlaylist } from '$lib/features/hierarchy/plays';
  import { current_play_folder_ids, play_folder_nodes } from '$lib/stores/plays';
  import { CirclePlusIcon, FolderIcon } from '@lucide/svelte';
  import { PlayPageBreadcrumb } from './Breadcrumb.svelte';

  let is_open_create_playlist = $state(false);
  let is_open_create_play_folder = $state(false);

  const current_play_folder_id = $derived($current_play_folder_ids[$current_play_folder_ids.length - 1]);
  const node = $derived.by(() => {
    const nodes = play_folder_nodes.get(current_play_folder_id);
    if (!nodes) {
      return [];
    }
    return nodes.children.sort((a, b) => a.sort_order - b.sort_order);
  });

  let rename_data = $state(undefined);
</script>

<CreatePlaylistDialog bind:open={is_open_create_playlist} />
<CreatePlayFolderDialog bind:open={is_open_create_play_folder} />
<RenameDialog bind:data={rename_data} />

<Separator class='mb-2' />
<PlayPageBreadcrumb />
<Separator class='my-2' />
<ScrollArea>
  <div class='w-full h-dvh relative'>
    <ContextMenu.Root>
      <ContextMenu.Trigger class='absolute w-full h-full z-10'>
      </ContextMenu.Trigger>
      <ContextMenu.Content>
        <ContextMenu.Item onclick={() => {
          is_open_create_playlist = true;
        }} class='flex items-center'>
          <IconText icon={CirclePlusIcon} text='Add Playlist' />
        </ContextMenu.Item>
        <ContextMenu.Item onclick={() => {
          is_open_create_play_folder = true;
        }} class='flex items-center'>
          <IconText icon={FolderIcon} text='Add Folder' />
        </ContextMenu.Item>
      </ContextMenu.Content>
    </ContextMenu.Root>

    <div class='absolute grid pb-50 p-4 gap-4 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5'>
      {#each node as child}
        {#if child.kind === 'Folder'}
          <HierarchyPlayerFolder folder_id={child.id} bind:rename_data />
        {:else if child.kind === 'Playlist'}
          <HierarchyPlaylist playlist_id={child.id} bind:rename_data />
        {/if}
      {/each}
    </div>
  </div>
</ScrollArea>
