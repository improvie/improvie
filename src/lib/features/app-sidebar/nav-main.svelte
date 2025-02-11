<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { Files, House, Icon, ListMusic, Search } from "lucide-svelte";

  const types: [string, typeof Icon, string][] = [
    ["home", House, "/"],
    ["items", Files, "/items"],
    ["playlists", ListMusic, "/playlists"],
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
    {#each types as [name, Icon, path]}
      <Sidebar.MenuButton
        isActive={page.url.pathname == path}
        onclick={() => {
          goto(path);
        }}
      >
        <Icon></Icon>
        <span class="capitalize">{name}</span>
      </Sidebar.MenuButton>
    {/each}
  </Sidebar.MenuItem>
</Sidebar.Menu>
