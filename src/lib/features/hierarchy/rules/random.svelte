<script lang='ts'>
  import type { RandomRule, RuleType } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Separator } from '$lib/components/ui/separator';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { CopyCheckIcon, CopyMinusIcon, ListPlusIcon, RepeatIcon, ShuffleIcon } from 'lucide-svelte';
  import { RuleNode } from '.';

  let { rule = $bindable() }: { rule: RandomRule } = $props();
  let open = $state(false);
  function add_rule(new_rule: RuleType) {
    rule.rules.push([new_rule, 1]);
  }
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<Card.Root>
  <Card.Content>
    <div class='flex'>
      <ShuffleIcon />
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
    <div class='block mt-2'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i][0]} />
      {/each}
    </div>
  </Card.Content>
</Card.Root>
