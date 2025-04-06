<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$lib/types/plays';
  import type { RuleType } from '$lib/types/rules';
  import { action_update_rules } from '$lib/action/rules';
  import { Button } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { setSlots } from '$lib/stores/index.svelte';
  import { clear_track, current_rules, set_current_rules } from '$lib/stores/track';
  import { ListPlusIcon, SquareMenuIcon } from '@lucide/svelte';
  import { PlaylistPlayer } from './Player.svelte';

  let { playlist = $bindable(), rules: prop_rules }: { playlist: Playlist; rules: RuleType[] } = $props();
  let rules = $state(prop_rules);
  $effect(() => {
    action_update_rules(playlist.id, rules);
    if (rules.length === 0) {
      clear_track();
    }
    else {
      $current_rules = rules;
      set_current_rules(rules);
    }
  });
  function add_rule(new_rule: RuleType) {
    rules.push(new_rule);
  }
  let open = $state(false);
  let player_open = $state(true);

  setSlots({
    prefix_pathname: '/editor',
    header,
  });
</script>

{#snippet header()}
  <Button
    type='button'
    onclick={() => {
      player_open = !player_open;
    }}
    variant='ghost'
    size='icon'
    class='mr-2'
  >
    <SquareMenuIcon />
  </Button>
{/snippet}

<CreateRuleDialog add_rule={add_rule} bind:open />

<div class='flex transition-all mx-4'>
  <Card.Root class='container w-2/3 select-none h-[90dvh] transition-all'>
    <div class='flex items-center my-2'>
      <h2 class='text-2xl'>Rules</h2>
      <button onclick={() => open = true} class='flex ml-4'><ListPlusIcon /> Add Rule</button>
    </div>
    <ScrollArea class='h-[70dvh]' orientation='both'>
      {#each rules as _, i}
        <RuleNode bind:rule={rules[i]} remove_rule={() => {
          rules = rules.filter((_, j) => i !== j);
        }} />
      {/each}
    </ScrollArea>
  </Card.Root>

  <PlaylistPlayer bind:open={player_open} />
</div>
