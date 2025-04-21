<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/Rule';
  import { action_update_rules } from '$lib/action/rules';
  import { Button } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { getLocalStorageOrDefault } from '$lib/local-storage';
  import { setSlots } from '$lib/stores/index.svelte';
  import { clear_track, current_rules, set_current_rules } from '$lib/stores/track';
  import { SquareMenuIcon } from '@lucide/svelte';
  import { EditorTracker } from './Tracker.svelte';

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
  let tracker_open = $state(getLocalStorageOrDefault('editor_tracker_open', 'true') === 'true');

  setSlots({
    prefix_pathname: '/editor',
    header,
  });
</script>

{#snippet header()}
  <Button
    type='button'
    onclick={() => {
      tracker_open = !tracker_open;
    }}
    variant='ghost'
    size='icon'
    class='mr-2'
  >
    <SquareMenuIcon />
  </Button>
{/snippet}

<CreateRuleDialog add_rule={add_rule} bind:open />

<div class='flex'>
  <Card.Root class='w-2/3 select-none'>
    <ScrollArea orientation='both'>
      <div class='w-full h-full p-6 flex flex-col gap-4'>
        {#each rules as _, i}
          <RuleNode bind:rule={rules[i]} remove_rule={() => {
            rules = rules.filter((_, j) => i !== j);
          }} />
        {/each}
      </div>
    </ScrollArea>
  </Card.Root>

  <EditorTracker bind:open={tracker_open} />
</div>
