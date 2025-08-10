<script lang='ts' module>
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import { defineSchema } from '../common/CommonFormSchema.svelte';

  export const FolderRuleSchema = defineSchema({
    folder_id: {
      type: 'folder_pick',
      label: 'folder',
    },
  });
  export type FolderRuleFormData = CommonSchemaToDataType<typeof FolderRuleSchema>;
</script>

<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const form = createForm(FolderRuleSchema);

  async function handleSubmit(data: FolderRuleFormData) {
    add_rule({
      type: 'Folder',
      data: {
        target_folder_id: data.folder_id,
      },
    });
  }

</script>

<CommonForm {form} schema={FolderRuleSchema} handle={handleSubmit} />
