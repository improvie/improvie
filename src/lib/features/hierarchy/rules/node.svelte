<script lang='ts'>
  import type { RuleType } from '$bindings/Rule';
  import { ContentRuleNode, LoopRuleNode, RandomRuleNode, RangeRuleNode } from '.';

  let {
    rule = $bindable(),
    remove_rule,
    dep,
  }: {
    rule: RuleType;
    remove_rule: () => void;
    dep: number;
  } = $props();
</script>

{#if rule.type === 'Content'}
  <ContentRuleNode {dep} bind:rule={rule.data} remove_rule={remove_rule} />
{:else if rule.type === 'Range'}
  <RangeRuleNode {dep} bind:rule={rule.data} remove_rule={remove_rule} />
{:else if rule.type === 'Random'}
  <RandomRuleNode {dep} bind:rule={rule.data} remove_rule={remove_rule} />
{:else if rule.type === 'Loop'}
  <LoopRuleNode {dep} bind:rule={rule.data} remove_rule={remove_rule} />
{/if}
