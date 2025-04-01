<script lang='ts'>
  import { current_track } from '$lib/stores/track';
  import Inner from './Inner.svelte';

</script>
{#if $current_track}
  <Inner track={$current_track} />
{/if}
