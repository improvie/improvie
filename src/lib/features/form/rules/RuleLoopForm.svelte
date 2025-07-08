<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import type { FormSchema } from '../common/CommonForm.svelte';
  import CommonForm, { createForm } from '../common/CommonForm.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const schema: FormSchema = {
    times: {
      type: 'uint',
      label: 'Times',
    },
  };
  const form = createForm(schema);

  async function handleSubmit(data: { times: number }) {
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
