<script lang="ts">
    import { AppBar, LightSwitch } from "@skeletonlabs/skeleton";
</script>

<AppBar
    gridColumns="grid-cols-3"
    slotDefault="place-self-center"
    slotTrail="place-content-end"
    background="bg-surface-900-100-token"
>
    <svelte:fragment slot="lead">
        <strong class="text-xl uppercase">Improvie</strong>
        <LightSwitch />
    </svelte:fragment>
    <div class="flex-grow">
        <span>Title</span>
        <span class="text-xs text-gray-500">Subtitle</span>
    </div>
    <svelte:fragment slot="trail">(action)</svelte:fragment>
</AppBar>
