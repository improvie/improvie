<script lang='ts'>
  import type { ContentRule } from '$bindings/rule';
  import type { ContentRuleFormData } from '$lib/features/form/rules/RuleContentForm.svelte';
  import ModifyElement from '$lib/components/element/ModifyElement.svelte';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import CommonFormDialog from '$lib/features/form/common/CommonFormDialog.svelte';
  import { createForm } from '$lib/features/form/common/CommonFormSchema.svelte';
  import { ContentRuleSchema } from '$lib/features/form/rules/RuleContentForm.svelte';
  import { contents } from '$lib/stores/items/content';
  import { HeadphonesIcon } from '@lucide/svelte';

  let {
    rule = $bindable(),
    remove_rule,
  }: {
    rule: ContentRule;
    remove_rule: () => void;
  } = $props();
  const content = $derived.by(() => {
    return contents.get(rule.content_id);
  });

  let modifyOpen = $state(false);
  const form = createForm(ContentRuleSchema);

  function onModifyOpen() {
    form.form.update((data) => {
      data.content_id = rule.content_id;
      return data;
    });
    modifyOpen = true;
  }

  async function handleSubmit(data: ContentRuleFormData) {
    rule.content_id = data.content_id;
    modifyOpen = false;
  }
</script>

<CommonFormDialog bind:open={modifyOpen} title='Modify Rule' {form} schema={ContentRuleSchema} handle={handleSubmit} />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible'>
    <HeadphonesIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <Card.Root class='min-w-48 sm:min-w-80 flex p-4'>
      <p class='line-clamp-2 break-words'>{content?.title || 'Loading... (maybe deleted)'}</p>
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
