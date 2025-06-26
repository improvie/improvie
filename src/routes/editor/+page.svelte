<script lang='ts'>
  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/rule';
  import { page } from '$app/state';
  import { action_get_rules } from '$lib/action/rules';
  import LoadSpinner from '$lib/components/LoadSpinner.svelte';
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
  <div class='w-full h-dvh flex flex-col justify-center items-center gap-4'>
    <div class='text-xl text-muted-foreground'>プレイリストが見つかりません</div>
    <div class='text-sm text-muted-foreground'>後でもう一度お試しください。</div>
  </div>
{:else}
  {#await playlist[1]}
    <div class='w-full h-dvh flex flex-col justify-center items-center gap-4'>
      <LoadSpinner class='h-8 w-8 text-primary' />
      <div class='text-lg text-muted-foreground'>プレイリストを読み込み中...</div>
    </div>
  {:then rules}
    <PlaylistInner playlist={playlist[0]} rules={rules} />
  {:catch e}
    <div class='w-full h-dvh flex flex-col justify-center items-center gap-4'>
      <div class='text-xl text-destructive'>エラーが発生しました</div>
      <div class='text-sm text-muted-foreground'>しばらく経ってから再度お試しください。</div>
    </div>
  {/await}
{/if}
