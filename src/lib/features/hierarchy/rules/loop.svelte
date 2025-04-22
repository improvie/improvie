<script lang='ts'>
  import type { LoopRule, RuleType } from '$bindings/Rule';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import RenameElement from '$lib/components/element/RenameElement.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RepeatIcon } from '@lucide/svelte';
  import { RuleNode } from '.';

  let {
    rule = $bindable(),
    remove_rule,
  }: {
    rule: LoopRule;
    remove_rule: () => void;
  } = $props();
  let open = $state(false);
  function add_rule(new_rule: RuleType) {
    rule.rules.push(new_rule);
  }
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible' oncontextmenu={e => e.stopPropagation()}>
    <RepeatIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <div class='absolute flex -translate-y-1/2 left-6'>
      <p>{rule.times}</p>
    </div>
    <Card.Root class='min-w-80 px-4 py-8 flex flex-col gap-6'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i]} remove_rule={() => {
          rule.rules = rule.rules.filter((_, j) => i !== j);
        }} />
      {/each}
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => open = true}>
      <RenameElement />
    </ContextMenu.Item>
    <ContextMenu.Item onclick={remove_rule}>
      <RemoveElement />
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
