<script lang='ts'>
  import type { LoopRule, RuleType } from '$bindings/Rule';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { ListPlusIcon, RepeatIcon } from '@lucide/svelte';
  import { RuleNode } from '.';

  let { rule = $bindable(), remove_rule }: { rule: LoopRule; remove_rule: () => void } = $props();
  let open = $state(false);
  function add_rule(new_rule: RuleType) {
    rule.rules.push(new_rule);
  }
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<Card.Root class='min-w-80'>
  <Card.Content>
    <ContextMenu.Root>
      <ContextMenu.Trigger>
        <div class='flex'>
          <RepeatIcon />
          <p class='mx-1'>{rule.times}</p>
          <button onclick={() => open = true} class='flex ml-8'><ListPlusIcon />Add Rule</button>
        </div>
      </ContextMenu.Trigger>
      <ContextMenu.Content>
        <ContextMenu.Item onclick={remove_rule}><p class='text-destructive'>Remove</p></ContextMenu.Item>
      </ContextMenu.Content>
    </ContextMenu.Root>
    <div>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i]} remove_rule={() => {
          rule.rules = rule.rules.filter((_, j) => i !== j);
        }} />
      {/each}
    </div>
  </Card.Content>
</Card.Root>
