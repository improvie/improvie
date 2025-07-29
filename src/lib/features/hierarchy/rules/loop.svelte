<script lang='ts'>
  import type { LoopRule, RuleType } from '$bindings/rule';
  import type { LoopRuleFormData } from '$lib/features/form/rules/RuleLoopForm.svelte';
  import ModifyElement from '$lib/components/element/ModifyElement.svelte';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import IconText from '$lib/components/IconText.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import CommonFormDialog from '$lib/features/form/common/CommonFormDialog.svelte';
  import { createForm } from '$lib/features/form/common/CommonFormSchema.svelte';
  import { LoopRuleSchema } from '$lib/features/form/rules/RuleLoopForm.svelte';
  import { ListPlusIcon, RepeatIcon } from '@lucide/svelte';
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

  let modifyOpen = $state(false);
  const form = createForm(LoopRuleSchema);

  function onModifyOpen() {
    form.form.update((data) => {
      data.times = rule.times;
      return data;
    });
    modifyOpen = true;
  }

  async function handleSubmit(data: LoopRuleFormData) {
    rule.times = data.times;
    modifyOpen = false;
  }
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<CommonFormDialog bind:open={modifyOpen} title='Modify Rule' {form} schema={LoopRuleSchema} handle={handleSubmit} />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible'>
    <RepeatIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <div class='absolute flex -translate-y-1/2 left-6'>
      <p>{rule.times}</p>
    </div>
    <Card.Root class='min-w-48 sm:min-w-80  px-4 py-8 flex flex-col gap-6'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i]} remove_rule={() => {
          rule.rules = rule.rules.filter((_, j) => i !== j);
        }} />
      {/each}
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => onModifyOpen()}>
      <ModifyElement />
    </ContextMenu.Item>
    <ContextMenu.Item onclick={() => open = true}>
      <IconText icon={ListPlusIcon} text='Add Rule' />
    </ContextMenu.Item>
    <ContextMenu.Item onclick={remove_rule}>
      <RemoveElement />
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
