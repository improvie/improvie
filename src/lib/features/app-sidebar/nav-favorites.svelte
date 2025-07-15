<script lang='ts'>
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import { useSidebar } from '$lib/components/ui/sidebar/index.js';
  import { select_playlist } from '$lib/stores/plays';
  import { favoritePlaylists, initFavoritePlaylist, removeFavoritePlaylist } from '$lib/stores/plays/favorite';
  import { playlists } from '$lib/stores/plays/playlist';
  import { StarIcon } from '@lucide/svelte';

  import Ellipsis from '@lucide/svelte/icons/ellipsis';
  import StarOff from '@lucide/svelte/icons/star-off';
  import { onMount } from 'svelte';

  onMount(() => {
    initFavoritePlaylist();
  });

  const favorites = $derived.by(() => {
    return $favoritePlaylists
      .map((id) => {
        return playlists.get(id);
      })
      .filter(v => v !== undefined);
  });

  const sidebar = useSidebar();

  const favoritePlaylistPath = '/favorite/playlists';
</script>

<Sidebar.Group>
  <Sidebar.GroupContent>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton
          isActive={page.url.pathname === favoritePlaylistPath}
          onclick={() => {
            goto(favoritePlaylistPath);
          }}
        >
          <StarIcon />
          <span class='line-clamp-1 text-wrap'>Favorite Playlists</span>
        </Sidebar.MenuButton>
        <Sidebar.MenuSub>
          {#each favorites as playlist}
            <Sidebar.MenuSubItem>
              <Sidebar.MenuSubButton onclick={() => select_playlist(playlist.id)}>
                <span>{playlist.title}</span>
              </Sidebar.MenuSubButton>
              <DropdownMenu.Root>
                <DropdownMenu.Trigger>
                  {#snippet child({ props })}
                    <Sidebar.MenuAction {...props}>
                      <Ellipsis />
                    </Sidebar.MenuAction>
                  {/snippet}
                </DropdownMenu.Trigger>
                <DropdownMenu.Content
                  class='w-56 rounded-lg'
                  side={sidebar.isMobile ? 'bottom' : 'right'}
                  align={sidebar.isMobile ? 'end' : 'start'}
                >
                  <DropdownMenu.Item
                    onclick={async () => {
                      await removeFavoritePlaylist(playlist.id);
                    }}
                  >
                    <StarOff class='text-muted-foreground' />
                    <span>Remove</span>
                  </DropdownMenu.Item>
                </DropdownMenu.Content>
              </DropdownMenu.Root>
            </Sidebar.MenuSubItem>
          {:else}
            <Sidebar.MenuSubItem>
              <Sidebar.MenuSubButton class='text-sidebar-foreground/70' aria-disabled>
                <span>Empty</span>
              </Sidebar.MenuSubButton>
            </Sidebar.MenuSubItem>
          {/each}
        </Sidebar.MenuSub>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.GroupContent>
</Sidebar.Group>
