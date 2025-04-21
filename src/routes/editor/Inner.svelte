<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/Rule';
  import { action_update_rules } from '$lib/action/rules';
  import { Button } from '$lib/components/ui/button';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { getLocalStorageOrDefault } from '$lib/local-storage';
  import { setSlots } from '$lib/stores/index.svelte';
  import { clear_track, current_rules, set_current_rules } from '$lib/stores/track';
  import { ListPlusIcon, SquareMenuIcon } from '@lucide/svelte';
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
  <Card.Root class='w-2/3 select-none z-0'>
    <ScrollArea orientation='both' class='relative w-full h-full'>
      <ContextMenu.Root>
        <ContextMenu.Trigger class='absolute w-full h-full z-[1]'>
        </ContextMenu.Trigger>
        <ContextMenu.Content>
          <ContextMenu.Item onclick={() => {
            open = true;
          }} class='flex items-center'>
            <ListPlusIcon class='mr-2 size-4' />Add Rule
          </ContextMenu.Item>
        </ContextMenu.Content>
      </ContextMenu.Root>

      <div class='absolute w-full flex flex-col gap-6 p-6'>
        {#each rules as _, i}
          <RuleNode dep={2} bind:rule={rules[i]} remove_rule={() => {
            rules = rules.filter((_, j) => i !== j);
          }} />
        {:else}
          <div class='flex items-center justify-center w-full h-full'>
            <p class='text-muted-foreground'>No rules. Open the context menu to add one.</p>
          </div>
        {/each}
      </div>
    </ScrollArea>
  </Card.Root>

  <EditorTracker bind:open={tracker_open} />
</div>
