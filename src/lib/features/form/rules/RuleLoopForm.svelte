<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm, defineSchema } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const schema = defineSchema({
    times: {
      type: 'uint',
      label: 'Times',
    },
  });
  const form = createForm(schema);
  type FormData = CommonSchemaToDataType<typeof schema>;

  async function handleSubmit(data: FormData) {
    add_rule({
      type: 'Loop',
      data: {
        times: data.times,
        rules: [],
      },
    });
  }

</script>

<CommonForm {form} {schema} handle={handleSubmit} />
