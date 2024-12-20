<script lang="ts">
    import { range } from "$lib/utils";
    import Icon from "@iconify/svelte";
    import {
        AppRail,
        AppRailTile,
        AppRailAnchor,
    } from "@skeletonlabs/skeleton";

    let currentTile: number = 0;
</script>

<AppRail
    background="bg-surface-900-100-token"
    class="ring-1 ring-surface-900-100-token"
>
    <svelte:fragment slot="lead">
        <AppRailAnchor title="Color Pallet">
            <svelte:fragment slot="lead">
                <Icon icon="mdi:color" width="36" height="36" />
            </svelte:fragment>
        </AppRailAnchor>
    </svelte:fragment>
    <svelte:fragment slot="default">
        <!-- this 160px is top and bottom elements size -->
        <div
            class="h-[calc(100vh-160px)] overflow-scroll ring-1 ring-surface-900-100-token"
        >
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
