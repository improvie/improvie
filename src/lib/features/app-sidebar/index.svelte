<script lang='ts'>
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import NavFavorites from '$lib/features/app-sidebar/nav-favorites.svelte';
  import NavMain from '$lib/features/app-sidebar/nav-main.svelte';
  import NavSecondary from '$lib/features/app-sidebar/nav-secondary.svelte';
</script>

<Sidebar.Root class='border-r-0 select-none'>
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
