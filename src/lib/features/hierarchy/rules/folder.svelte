<script lang='ts'>
  import type { FolderRule } from '$bindings/rule';
  import type { FolderRuleFormData } from '$lib/features/form/rules/RuleFolderForm.svelte';
  import ModifyElement from '$lib/components/element/ModifyElement.svelte';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import CommonFormDialog from '$lib/features/form/common/CommonFormDialog.svelte';
  import { createForm } from '$lib/features/form/common/CommonFormSchema.svelte';
  import { FolderRuleSchema } from '$lib/features/form/rules/RuleFolderForm.svelte';
  import { folders } from '$lib/stores/items/folder';
  import { FolderIcon } from '@lucide/svelte';

  let {
    rule = $bindable(),
    remove_rule,
  }: {
    rule: FolderRule;
    remove_rule: () => void;
  } = $props();
  const folder = $derived.by(() => {
    return folders.get(rule.target_folder_id);
  });

  let modifyOpen = $state(false);
  const form = createForm(FolderRuleSchema);

  function onModifyOpen() {
    form.form.update((data) => {
      data.folder_id = rule.target_folder_id;
      return data;
    });
    modifyOpen = true;
  }

  async function handleSubmit(data: FolderRuleFormData) {
    rule.target_folder_id = data.folder_id;
    modifyOpen = false;
  }
</script>

<CommonFormDialog bind:open={modifyOpen} title='Modify Rule' {form} schema={FolderRuleSchema} handle={handleSubmit} />

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible'>
    <FolderIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <Card.Root class='min-w-48 sm:min-w-80 flex p-4'>
      <p class='line-clamp-2 break-words'>{folder?.title || 'Loading... (maybe deleted)'}</p>
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
