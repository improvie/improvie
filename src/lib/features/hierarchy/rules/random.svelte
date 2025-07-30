<script lang='ts'>
  import type { RandomRule, RuleType } from '$bindings/rule';
  import type { RandomRuleFormData } from '$lib/features/form/rules/RuleRandomForm.svelte';
  import ModifyElement from '$lib/components/element/ModifyElement.svelte';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import IconText from '$lib/components/IconText.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { Separator } from '$lib/components/ui/separator';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import CommonFormDialog from '$lib/features/form/common/CommonFormDialog.svelte';
  import { createForm } from '$lib/features/form/common/CommonFormSchema.svelte';
  import { RandomRuleSchema } from '$lib/features/form/rules/RuleRandomForm.svelte';
  import { CopyCheckIcon, CopyMinusIcon, ListPlusIcon, RepeatIcon, ShuffleIcon } from '@lucide/svelte';
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

  let modifyOpen = $state(false);
  const form = createForm(RandomRuleSchema);

  function onModifyOpen() {
    form.form.update((data) => {
      data.times = rule.times;
      data.duplicate = rule.duplicate;
      return data;
    });
    modifyOpen = true;
  }

  async function handleSubmit(data: RandomRuleFormData) {
    rule.times = data.times;
    rule.duplicate = data.duplicate;
    modifyOpen = false;
  }

</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<CommonFormDialog bind:open={modifyOpen} title='Modify Rule' {form} schema={RandomRuleSchema} handle={handleSubmit} />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible'>
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
    <Card.Root class='min-w-48 sm:min-w-80 px-4 py-8 flex flex-col gap-6'>
      {#each rule.rules as _, i}
        <RuleNode bind:rule={rule.rules[i][0]} remove_rule={() => {
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
