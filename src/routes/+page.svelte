<script lang="ts">
  import HomePage from "$lib/pages/HomePage.svelte";
  import ItemsPage from "$lib/pages/ItemsPage.svelte";
  import PlaylistsPage from "$lib/pages/PlaylistsPage.svelte";
  import { main_menu_store } from "$lib/stores/MainMenu";
</script>

{#if $main_menu_store == "home"}
  <HomePage />
{:else if $main_menu_store == "items"}
  <ItemsPage />
{:else if $main_menu_store == "playlists"}
  <PlaylistsPage />
{/if}
