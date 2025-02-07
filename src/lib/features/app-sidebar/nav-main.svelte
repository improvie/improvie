<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { main_menu_store, type MainMenuType } from "$lib/stores/MainMenu";
  import { Files, House, Icon, ListMusic, Search } from "lucide-svelte";

  const types: [MainMenuType, typeof Icon][] = [
    ["home", House],
    ["items", Files],
    ["playlists", ListMusic],
  ];
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <Sidebar.MenuButton>
      <Search />
      <span>Search</span>
    </Sidebar.MenuButton>
  </Sidebar.MenuItem>
  <Sidebar.MenuItem>
    {#each types as [type, icon]}
      <Sidebar.MenuButton
        isActive={$main_menu_store == type}
        onclick={() => ($main_menu_store = type)}
      >
        <svelte:component this={icon} />
        <span class="capitalize">{type}</span>
      </Sidebar.MenuButton>
    {/each}
  </Sidebar.MenuItem>
</Sidebar.Menu>
