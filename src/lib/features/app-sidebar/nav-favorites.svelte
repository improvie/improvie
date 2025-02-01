<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { useSidebar } from "$lib/components/ui/sidebar/index.js";
  import { getPlaylist } from "$lib/stores/playlist";
  import {
    favoritePlaylists,
    initFavoritePlaylist,
    removeFavoritePlaylist,
  } from "$lib/stores/playlist/favorite";
  import Ellipsis from "lucide-svelte/icons/ellipsis";
  import StarOff from "lucide-svelte/icons/star-off";
  import { derived } from "svelte/store";

  initFavoritePlaylist();

  const favorites = derived(favoritePlaylists, ($favoritePlaylists) => {
    return $favoritePlaylists
      .map((id) => getPlaylist(id))
      .filter((v) => v != undefined);
  });

  const sidebar = useSidebar();
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
  <Sidebar.GroupLabel>Favorites</Sidebar.GroupLabel>
  <Sidebar.Menu>
    {#each $favorites as playlist}
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <span>{playlist.title}</span>
        </Sidebar.MenuButton>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            <Sidebar.MenuAction showOnHover>
              <Ellipsis />
              <span class="sr-only">More</span>
            </Sidebar.MenuAction>
          </DropdownMenu.Trigger>
          <DropdownMenu.Content
            class="w-56 rounded-lg"
            side={sidebar.isMobile ? "bottom" : "right"}
            align={sidebar.isMobile ? "end" : "start"}
          >
            <DropdownMenu.Item
              onclick={async () => {
                await removeFavoritePlaylist(playlist.id);
              }}
            >
              <StarOff class="text-muted-foreground" />
              <span>Remove from Favorites</span>
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </Sidebar.MenuItem>
    {:else}
      <Sidebar.MenuItem>
        <Sidebar.MenuButton class="text-sidebar-foreground/70" aria-disabled>
          <span>Empty</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    {/each}
  </Sidebar.Menu>
</Sidebar.Group>
