<script lang='ts'>
  import { contents } from '$lib/stores/items/content';
  import { current_track } from '$lib/stores/track';
  import Inner from './Inner.svelte';

  let track = $derived.by(() => {
    if (!$current_track) {
      return undefined;
    }
    return $contents.get($current_track);
  });
</script>
{#if track}
  <Inner bind:track />
{/if}
