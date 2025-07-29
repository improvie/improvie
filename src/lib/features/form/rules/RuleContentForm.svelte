<script lang='ts' module>
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import { defineSchema } from '../common/CommonFormSchema.svelte';

  export const ContentRuleSchema = defineSchema({
    content_id: {
      type: 'content_pick',
      label: 'Content',
    },
  });
  export type ContentRuleFormData = CommonSchemaToDataType<typeof ContentRuleSchema>;
</script>

<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const form = createForm(ContentRuleSchema);

  async function handleSubmit(data: ContentRuleFormData) {
    add_rule({
      type: 'Content',
      data: {
        content_id: data.content_id,
      },
    });
  }

</script>

<CommonForm {form} schema={ContentRuleSchema} handle={handleSubmit} />
