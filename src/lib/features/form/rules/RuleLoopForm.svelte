<script lang='ts' module>
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import { defineSchema } from '../common/CommonFormSchema.svelte';

  export const LoopRuleSchema = defineSchema({
    times: {
      type: 'uint',
      label: 'Times',
      props: {
        description: '合計何回ループするか',
      },
    },
  });
  export type LoopRuleFormData = CommonSchemaToDataType<typeof LoopRuleSchema>;
</script>

<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();
  const form = createForm(LoopRuleSchema);

  async function handleSubmit(data: LoopRuleFormData) {
    add_rule({
      type: 'Loop',
      data: {
        times: data.times,
        rules: [],
      },
    });
  }

</script>

<CommonForm {form} schema={LoopRuleSchema} handle={handleSubmit} />
