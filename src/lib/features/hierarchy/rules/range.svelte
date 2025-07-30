<script lang='ts'>
  import type { RangeRule } from '$bindings/rule';
  import type { RangeRuleFormData } from '$lib/features/form/rules/RuleRangeForm.svelte';
  import ModifyElement from '$lib/components/element/ModifyElement.svelte';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import CommonFormDialog from '$lib/features/form/common/CommonFormDialog.svelte';
  import { createForm } from '$lib/features/form/common/CommonFormSchema.svelte';
  import { RangeRuleSchema } from '$lib/features/form/rules/RuleRangeForm.svelte';
  import { contents } from '$lib/stores/items/content';
  import { ChevronsUpIcon } from '@lucide/svelte';

  let {
    rule = $bindable(),
    remove_rule,
  }: {
    rule: RangeRule;
    remove_rule: () => void;
  } = $props();
  const content = $derived.by(() => {
    return contents.get(rule.content_id);
  });

  let modifyOpen = $state(false);
  const form = createForm(RangeRuleSchema);

  function onModifyOpen() {
    form.form.update((data) => {
      data.content_id = rule.content_id;
      data.range = [rule.range_start ?? 0, rule.range_end ?? 0];
      return data;
    });
    modifyOpen = true;
  }

  async function handleSubmit(data: RangeRuleFormData) {
    rule.content_id = data.content_id;
    const range = data.range;
    rule.range_start = range[0];
    rule.range_end = range[1];
    modifyOpen = false;
  }
</script>

<CommonFormDialog bind:open={modifyOpen} title='Modify Rule' {form} schema={RangeRuleSchema} handle={handleSubmit} />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible'>
    <ChevronsUpIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <Card.Root class='min-w-48 sm:min-w-80'>
      <Card.Content class='flex'>
        <p>{content?.title || 'Loading... (maybe deleted)'}</p>
        <p>{rule.range_start}s - {rule.range_end}s</p>
      </Card.Content>
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={() => onModifyOpen()}>
      <ModifyElement />
    </ContextMenu.Item>
    <ContextMenu.Item onclick={remove_rule}>
      <RemoveElement />
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
