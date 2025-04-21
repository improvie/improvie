<script lang='ts'>
  import type { RandomRule, RuleType } from '$bindings/Rule';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { Separator } from '$lib/components/ui/separator';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { CopyCheckIcon, CopyMinusIcon, ListPlusIcon, RepeatIcon, ShuffleIcon } from '@lucide/svelte';
  import { RuleNode } from '.';

  let {
    rule = $bindable(),
    remove_rule,
    dep,
  }: {
    rule: RandomRule;
    remove_rule: () => void;
    dep: number;
  } = $props();
  let open = $state(false);

  function add_rule(new_rule: RuleType) {
    rule.rules.push([new_rule, 1]);
  }

</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible' style={{
    'z-index': dep,
  }} oncontextmenu={e => e.stopPropagation()}>
    <ShuffleIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <Card.Root class='min-w-80'>
      <div class='flex'>
        <Separator orientation='vertical' class='mx-1' />
        {#if rule.duplicate}
          <CopyCheckIcon />
        {:else}
          <CopyMinusIcon />
        {/if}
        <Separator orientation='vertical' class='mx-1' />
        <RepeatIcon />
        <p class='mx-1'>{rule.times}</p>
        <button onclick={() => open = true} class='flex ml-8'><ListPlusIcon />Add Rule</button>
      </div>
      <div class='p-4 flex flex-col gap-4'>
        {#each rule.rules as _, i}
          <RuleNode dep={dep + 1} bind:rule={rule.rules[i][0]} remove_rule={() => {
            rule.rules = rule.rules.filter((_, j) => i !== j);
          }} />
        {/each}
      </div>
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={remove_rule}><p class='text-destructive'>Remove</p></ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
