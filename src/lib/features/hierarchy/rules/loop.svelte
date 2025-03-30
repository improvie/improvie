<script lang='ts'>
  import type { LoopRule } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { ListPlusIcon, RepeatIcon } from 'lucide-svelte';
  import { RuleNode } from '.';

  let { rule = $bindable() }: { rule: LoopRule } = $props();
  let open = $state(false);
</script>

<CreateRuleDialog bind:rules={rule.rules} bind:open />

<Card.Root>
  <Card.Content>
    <div class='flex mb-2'>
      <RepeatIcon />
      <p class='mx-2'>{rule.times}</p>
      <button onclick={() => open = true} class='flex ml-4'><ListPlusIcon />Add Rule</button>
    </div>
    <div class='block'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i]} />
      {/each}
    </div>
  </Card.Content>
</Card.Root>
