<script lang="ts">
  import { range } from "$lib/utils";
  import Icon from "@iconify/svelte";
  import { AppRail, AppRailAnchor, AppRailTile } from "@skeletonlabs/skeleton";
  import ThemeChanger from "./ThemeChanger.svelte";

  let currentTile: number = 0;
</script>

<AppRail background="bg-surface-50-900-token" class="ring-1">
  <svelte:fragment slot="lead">
    <AppRailAnchor title="Color Pallet">
      <svelte:fragment slot="lead">
        <ThemeChanger />
      </svelte:fragment>
    </AppRailAnchor>
  </svelte:fragment>
  <svelte:fragment slot="default">
    <!-- this 160px is top and bottom elements size -->
    <div class="h-[calc(100vh-160px)] overflow-scroll ring-1">
      {#each range(1, 30) as i}
        <AppRailTile
          bind:group={currentTile}
          name="tile-{i}"
          value={i}
          title="tile-{i}"
        >
          <svelte:fragment slot="lead">(icon)</svelte:fragment>
          <span>Tile {i}</span>
        </AppRailTile>
      {/each}
    </div>
  </svelte:fragment>
  <svelte:fragment slot="trail">
    <AppRailAnchor title="Settings">
      <svelte:fragment slot="lead">
        <Icon icon="mdi:cog" width="36" height="36" />
      </svelte:fragment>
    </AppRailAnchor>
  </svelte:fragment>
</AppRail>
