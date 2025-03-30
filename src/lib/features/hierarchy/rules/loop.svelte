<script lang='ts'>
  import type { LoopRule, RuleType } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { ListPlusIcon, RepeatIcon } from 'lucide-svelte';
  import { RuleNode } from '.';

  let { rule = $bindable() }: { rule: LoopRule } = $props();
  let open = $state(false);
  function add_rule(new_rule: RuleType) {
    rule.rules.push(new_rule);
  }
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<Card.Root>
  <Card.Content>
    <div class='flex'>
      <RepeatIcon />
      <p class='mx-1'>{rule.times}</p>
      <button onclick={() => open = true} class='flex ml-8'><ListPlusIcon />Add Rule</button>
    </div>
    <div class='block mt-2'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i]} />
      {/each}
    </div>
  </Card.Content>
</Card.Root>
