<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { useSidebar } from "$lib/components/ui/sidebar/index.js";
  import { removeFavoritePlaylist } from "$lib/stores/playlist/favorite";
  import type { Playlist } from "$lib/types/playlist";
  import Ellipsis from "lucide-svelte/icons/ellipsis";
  import StarOff from "lucide-svelte/icons/star-off";

  const favorites: Playlist[] = $state([]);

  const sidebar = useSidebar();
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
  <Sidebar.GroupLabel>Favorites</Sidebar.GroupLabel>
  <Sidebar.Menu>
    {#each favorites as item (item.id)}
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <span>{item.emoji || " "}</span>
          <span>{item.title}</span>
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
                await removeFavoritePlaylist(item.id);
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
