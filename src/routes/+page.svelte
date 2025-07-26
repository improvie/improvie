<script lang='ts'>
  import { action_get_recent_contents, action_get_recent_playlists } from '$lib/action/recents';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte';
  import { contents } from '$lib/stores/items/content';
  import { get_playlist_thumbnail_path, playlists } from '$lib/stores/plays/playlist';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { CirclePlayIcon } from '@lucide/svelte';

  let recentContents: string[] = $state([]);
  let longContents: string[] = $state([]);
  let recentPlaylists: string[] = $state([]);

  async function init() {
    recentContents = await action_get_recent_contents(30, { Max: 1800 });
    longContents = await action_get_recent_contents(10, { Min: 1800 });
    recentPlaylists = await action_get_recent_playlists(10);
  }

  $effect(() => {
    init();
  });

  function onclickContent(contentId: string) {
    tracker.set_single_content(contentId);
    init();
  }

  function onclickPlaylist(playlistId: string) {
    tracker.set_rules_by_type(playlistId);
    init();
  }

  function playAllContents() {
    tracker.set_multiple_contents(recentContents);
  }

</script>

<ScrollArea>
  <div class='flex flex-col w-full h-dvh items-center gap-4'>
    <div class='flex flex-col container'>
      <div class='flex justify-between p-2'>
        <h2 class='text-3xl font-bold'>
          最近みたもの
        </h2>
        <Button variant='outline' onclick={() => playAllContents()}>
          <CirclePlayIcon />
          全て再生
        </Button>
      </div>
      <ScrollArea class='whitespace-nowrap rounded-md border container' orientation='horizontal'>
        <div class='h-74 flex flex-col flex-wrap gap-6 p-4 w-fit'>
          {#each recentContents as contentId}
            {@const content = contents.get(contentId)}
            {#if content !== undefined}
              <div
                role={content.title}
                class='h-18 w-90 gap-1 flex flex-row'
                onclick={() => onclickContent(contentId)}
              >
                <ImageLoader local direction='vertical' class='rounded-sm border' src={content.thumbnail_path} />
                <p class='text-wrap line-clamp-3'>{content.title}</p>
              </div>
            {/if}
          {:else}
            <p class='text-muted-foreground text-center'>最近みたものはありません</p>
          {/each}
        </div>
      </ScrollArea>
    </div>
    {#if longContents.length > 0}
      <div class='flex flex-col container'>
        <h2 class='text-3xl m-2 font-bold'>
          長時間動画
        </h2>
        <ScrollArea class='whitespace-nowrap rounded-md border container' orientation='horizontal'>
          <div class='flex gap-6 p-4'>
            {#each longContents as contentId}
              {@const content = contents.get(contentId)}
              {#if content !== undefined}
                <div
                  role={content.title}
                  class='h-47 w-64 gap-2 flex flex-col'
                  onclick={() => onclickContent(contentId)}
                >
                  <ImageLoader local direction='vertical' class='rounded-sm border' src={content.thumbnail_path} />
                  <p class='text-wrap line-clamp-1'>{content.title}</p>
                </div>
              {/if}
            {:else}
              <p class='text-muted-foreground text-center'>最近みたものはありません</p>
            {/each}
          </div>
        </ScrollArea>
      </div>
    {/if}
    {#if recentPlaylists.length > 0}
      <div class='flex flex-col container'>
        <h2 class='text-3xl m-2 font-bold'>
          最近みたプレイリスト
        </h2>
        <ScrollArea class='whitespace-nowrap rounded-md border container' orientation='horizontal'>
          <div class='flex gap-6 p-4'>
            {#each recentPlaylists as playlistId}
              {@const playlist = playlists.get(playlistId)}
              {#if playlist !== undefined}
                {@const thumbnail_path = get_playlist_thumbnail_path(playlist, contents)}
                <div
                  role={playlist.title}
                  class='h-59 w-80 gap-2 flex flex-col'
                  onclick={() => onclickPlaylist(playlistId)}
                >
                  <div class='h-45 w-full'>
                    {#await thumbnail_path}
                      <ImageLoader loading class='rounded-sm border' src={null} />
                    {:then path}
                      <ImageLoader local class='rounded-sm border' src={path} />
                    {:catch}
                      <ImageLoader failed class='rounded-sm border' src={null} />
                    {/await}
                  </div>
                  <p class='text-wrap line-clamp-2'>{playlist.title}</p>
                </div>
              {/if}
            {/each}
          </div>
        </ScrollArea>
      </div>
    {/if}

    <div class='container min-h-40'></div>
  </div>
</ScrollArea>
