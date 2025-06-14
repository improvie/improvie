<script lang='ts'>
  import { tracker } from '$lib/stores/tracker.svelte';
</script>

<svelte:window onkeydown={(event) => {
  if (document.activeElement?.tagName === 'INPUT' || document.activeElement?.tagName === 'TEXTAREA') {
    return;
  }
  event.preventDefault();
  if (event.key === 'o') {
    tracker.external_open = !tracker.external_open;
  }
  if (event.key === 'k') {
    tracker.toggle_pause();
  }
}} />
