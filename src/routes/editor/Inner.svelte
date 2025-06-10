<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/rule';
  import { action_update_rules } from '$lib/action/rules';
  import IconText from '$lib/components/IconText.svelte';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn } from '$lib/utils';
  import { ListPlusIcon } from '@lucide/svelte';

  let { playlist = $bindable(), rules: prop_rules }: { playlist: Playlist; rules: RuleType[] } = $props();
  let rules = $state(prop_rules);
  $effect(() => {
    action_update_rules(playlist.id, rules);
    tracker.set_rules_by_type(rules);
  });
  function add_rule(new_rule: RuleType) {
    rules.push(new_rule);
  }
  let open = $state(false);
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<ScrollArea orientation='both' class={cn('w-full h-dvh relative z-0', open && 'sm:w-2/3')}>
  <ContextMenu.Root>
    <ContextMenu.Trigger>
      <div class='w-full flex flex-col gap-6 p-6'>
        {#each rules as _, i}
          <RuleNode bind:rule={rules[i]} remove_rule={() => {
            rules = rules.filter((_, j) => i !== j);
          }} />
        {:else}
          <div class='flex items-center justify-center w-full h-full'>
            <p class='text-muted-foreground'>No rules. Open the context menu to add one.</p>
          </div>
        {/each}
      </div>
      <div class='min-h-80'></div>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={() => {
        open = true;
      }}>
        <IconText icon={ListPlusIcon} text='Add Rule' />
      </ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
</ScrollArea>
