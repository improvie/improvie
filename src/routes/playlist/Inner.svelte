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

  // TODO: reference array になっていない rulesに変更が適用されていない
  let { playlist = $bindable(), rules = $bindable() }: { playlist: Playlist; rules: { rules: RuleType[] } } = $props();
  let open = $state(false);
</script>

<CreateRuleDialog bind:target={rules} bind:open />

<Card.Root class='container w-2/3 mx-auto'>
  <Card.Header>
    <Card.Title>Title: {playlist.title}</Card.Title>
    {#if playlist.description}
      <Card.Description>Desc: {playlist.description}</Card.Description>
    {/if}
  </Card.Header>
  <Card.Content>
    <div class='flex items-center'>
      <h2 class='text-2xl'>Rules</h2>
      <button onclick={() => open = true} class='flex ml-4'><ListPlusIcon /> Add Rule</button>
    </div>
    {#each rules.rules as _, i}
      <RuleNode bind:rule={rules.rules[i]} />
    {/each}
  </Card.Content>
</Card.Root>
