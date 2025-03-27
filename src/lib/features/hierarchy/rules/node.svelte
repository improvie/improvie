<script lang='ts'>
  import type { RuleType } from '$lib/types/rules';
  import { ContentRuleNode, LoopRuleNode, RandomRuleNode, RangeRuleNode } from '.';

  let { rule = $bindable() }: { rule: RuleType } = $props();
</script>

{#if rule.type === 'Content'}
  <ContentRuleNode bind:rule={rule.data} />
{:else if rule.type === 'Range'}
  <RangeRuleNode bind:rule={rule.data} />
{:else if rule.type === 'Random'}
  <RandomRuleNode bind:rule={rule.data} />
{:else if rule.type === 'Loop'}
  <LoopRuleNode bind:rule={rule.data} />
{/if}
