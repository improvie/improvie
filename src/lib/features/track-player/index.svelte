<script lang='ts'>
  import { contents } from '$lib/stores/items/content';
  import { current_track_id } from '$lib/stores/track';
  import Inner from './Inner.svelte';

  let track = $derived.by(() => {
    if ($current_track_id === undefined) {
      return undefined;
    }
    return $contents.get($current_track_id);
  });
</script>
{#if track}
  {#key track.id}
    <Inner bind:track />
  {/key}
{/if}
