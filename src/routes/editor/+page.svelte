<script lang='ts'>
  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/rule';
  import { page } from '$app/state';
  import { action_get_rules } from '$lib/action/rules';
  import { playlists } from '$lib/stores/plays/playlist';
  import { PlaylistInner } from './Inner.svelte';

  const playlist: [ Playlist, Promise<RuleType[]> ] | undefined = $derived.by(() => {
    const current_playlist_id = page.url.searchParams.get('id');
    if (current_playlist_id === null) {
      return undefined;
    }
    const playlist = playlists.get(current_playlist_id);
    if (playlist === undefined) {
      return undefined;
    }
    return [playlist, action_get_rules(playlist.id)];
  });

</script>

{#if playlist === undefined}
  <div class='w-full h-dvh flex justify-center items-center'>Playlist not found. please try again later.</div>
{:else}
  {#await playlist[1]}
    <div class='w-full h-dvh flex justify-center items-center'>Loading...</div>
  {:then rules}
    <PlaylistInner playlist={playlist[0]} rules={rules} />
  {:catch e}
    {console.error(e)}
    <div class='w-full h-dvh flex justify-center items-center'>Your request failed. Please try again later.</div>
  {/await}
{/if}
