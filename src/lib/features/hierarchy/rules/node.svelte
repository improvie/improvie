<script lang='ts'>
  import type { RuleType } from '$bindings/Rule';
  import { ContentRuleNode, LoopRuleNode, RandomRuleNode, RangeRuleNode } from '.';

  let { rule = $bindable(), remove_rule }: { rule: RuleType; remove_rule: () => void } = $props();
</script>

{#if rule.type === 'Content'}
  <ContentRuleNode bind:rule={rule.data} remove_rule={remove_rule} />
{:else if rule.type === 'Range'}
  <RangeRuleNode bind:rule={rule.data} remove_rule={remove_rule} />
{:else if rule.type === 'Random'}
  <RandomRuleNode bind:rule={rule.data} remove_rule={remove_rule} />
{:else if rule.type === 'Loop'}
  <LoopRuleNode bind:rule={rule.data} remove_rule={remove_rule} />
{/if}
