<script lang='ts'>
  import { tracker } from '$lib/stores/tracker.svelte';
  import Inner from './Inner.svelte';

  let track = $derived(tracker.get_current_content());
</script>

{#if track}
  <Inner bind:track />
{/if}
