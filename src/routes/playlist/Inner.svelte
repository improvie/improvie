<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$lib/types/plays';
  import type { RuleType } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import { RuleNode } from '$lib/features/hierarchy/rules';

  let { playlist = $bindable(), rules = $bindable() }: { playlist: Playlist; rules: RuleType[] } = $props();
</script>

<Card.Root class='container w-2/3 mx-auto'>
  <Card.Header>
    <Card.Title>Title: {playlist.title}</Card.Title>
    {#if playlist.description}
      <Card.Description>Desc: {playlist.description}</Card.Description>
    {/if}
  </Card.Header>
  <Card.Content>
    {#each rules as _, i}
      <RuleNode bind:rule={rules[i]} />
    {/each}
  </Card.Content>
</Card.Root>
