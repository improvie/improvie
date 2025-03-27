<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$lib/types/plays';
  import type { RuleType } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { ListPlusIcon } from 'lucide-svelte';

  let { playlist = $bindable(), rules = $bindable() }: { playlist: Playlist; rules: RuleType[] } = $props();
  let target = $state<RuleType[] | undefined>(undefined);
</script>

<CreateRuleDialog bind:target />

<Card.Root class='container w-2/3 mx-auto'>
  <Card.Header>
    <Card.Title>Title: {playlist.title}</Card.Title>
    {#if playlist.description}
      <Card.Description>Desc: {playlist.description}</Card.Description>
    {/if}
  </Card.Header>
  <Card.Content>
    <div class='flex items-end'>
      <h2 class='text-2xl'>Rules</h2>
      <button onclick={() => target = rules} class='flex ml-4'><ListPlusIcon /> Add Rule</button>
    </div>
    {#each rules as _, i}
      <RuleNode bind:rule={rules[i]} />
    {/each}
  </Card.Content>
</Card.Root>
