<script lang='ts'>
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import RenameDialog from '$lib/features/dialog/RenameDialog.svelte';
  import { HierarchyPlaylist } from '$lib/features/hierarchy/plays';
  import { favoritePlaylists } from '$lib/stores/plays/favorite';

  let rename_data = $state(undefined);
</script>

<RenameDialog bind:data={rename_data} />

<ScrollArea>
  <div class='w-full h-dvh relative'>
    <div class='absolute top-0 pb-50 w-full grid p-4 gap-4 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5'>
      {#each $favoritePlaylists as playlistId}
        <HierarchyPlaylist playlist_id={playlistId} bind:rename_data />
      {/each}
    </div>
  </div>
</ScrollArea>
