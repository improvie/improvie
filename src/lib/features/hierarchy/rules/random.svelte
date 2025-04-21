<script lang='ts'>
  import type { RandomRule, RuleType } from '$bindings/Rule';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { Separator } from '$lib/components/ui/separator';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { CopyCheckIcon, CopyMinusIcon, ListPlusIcon, RepeatIcon, ShuffleIcon, TrashIcon } from '@lucide/svelte';
  import { RuleNode } from '.';

  let {
    rule = $bindable(),
    remove_rule,
  }: {
    rule: RandomRule;
    remove_rule: () => void;
  } = $props();
  let open = $state(false);

  function add_rule(new_rule: RuleType) {
    rule.rules.push([new_rule, 1]);
  }

</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible' oncontextmenu={e => e.stopPropagation()}>
    <ShuffleIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <div class='absolute flex -translate-y-1/2 left-6'>
      {#if rule.duplicate}
        <CopyCheckIcon />
      {:else}
        <CopyMinusIcon />
      {/if}
      <Separator orientation='vertical' class='mx-1' />
      <RepeatIcon />
      <p class='mx-1'>{rule.times}</p>
    </div>
    <Card.Root class='min-w-80 px-4 py-8 flex flex-col gap-6'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i][0]} remove_rule={() => {
          rule.rules = rule.rules.filter((_, j) => i !== j);
        }} />
      {/each}
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => open = true}>
      <ListPlusIcon />Add Rule
    </ContextMenu.Item>
    <ContextMenu.Item onclick={remove_rule} class='text-destructive'>
      <TrashIcon />Remove
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
