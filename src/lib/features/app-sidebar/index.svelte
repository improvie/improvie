<script lang="ts">
  import NavFavorites from '$lib/features/app-sidebar/nav-favorites.svelte';
  import NavMain from '$lib/features/app-sidebar/nav-main.svelte';
  import NavSecondary from '$lib/features/app-sidebar/nav-secondary.svelte';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
</script>

<Sidebar.Root class="border-r-0">
  <Sidebar.Header>
    <NavMain />
  </Sidebar.Header>
  <Sidebar.Content>
    <NavFavorites />
  </Sidebar.Content>
  <Sidebar.Footer>
    <NavSecondary />
  </Sidebar.Footer>
</Sidebar.Root>
