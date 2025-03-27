<script lang='ts'>
  import type { Playlist } from '$lib/types/plays';
  import { current_playlist_id } from '$lib/stores/plays';
  import { playlists } from '$lib/stores/plays/playlist';
  import { PlaylistInner } from './Inner.svelte';

  let playlist: Playlist | undefined = $derived.by(() => {
    return $playlists.get($current_playlist_id);
  });
</script>

{#if playlist === undefined}
  <div>Loading...</div>
{:else}
  <PlaylistInner bind:playlist />
{/if}
