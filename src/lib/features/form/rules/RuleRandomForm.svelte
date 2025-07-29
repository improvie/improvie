<script lang='ts' module>
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import { defineSchema } from '../common/CommonFormSchema.svelte';

  export const RandomRuleSchema = defineSchema({
    times: {
      type: 'uint',
      label: 'Times',
      props: {
        description: '合計何回ループするか',
      },
    },
    duplicate: {
      type: 'checkbox',
      label: 'Duplicate',
      props: {
        description: '同じ内容が出るのを許容するか',
      },
    },
  });
  export type RandomRuleFormData = CommonSchemaToDataType<typeof RandomRuleSchema>;
</script>

<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const form = createForm(RandomRuleSchema);

  async function handleSubmit(data: RandomRuleFormData) {
    add_rule({
      type: 'Random',
      data: {
        duplicate: data.duplicate,
        times: data.times,
        rules: [],
      },
    });
  }
</script>

<CommonForm {form} schema={RandomRuleSchema} handle={handleSubmit} />
