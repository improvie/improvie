<script module>
  export { default as PlaylistInner } from './Inner.svelte';

</script>

<script lang='ts'>
  import type { Playlist } from '$lib/types/plays';

  let { playlist = $bindable() }: { playlist: Playlist } = $props();
</script>

<div>
  <h1>{playlist.title}</h1>
</div>
