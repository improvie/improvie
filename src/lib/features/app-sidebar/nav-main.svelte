<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { House, Search } from "lucide-svelte";
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <Sidebar.MenuButton>
      <Search />
      <span>Search</span>
    </Sidebar.MenuButton>
  </Sidebar.MenuItem>
  <Sidebar.MenuItem>
    <Sidebar.MenuButton isActive={true}>
      <House />
      <span>Home</span>
    </Sidebar.MenuButton>
  </Sidebar.MenuItem>
</Sidebar.Menu>
