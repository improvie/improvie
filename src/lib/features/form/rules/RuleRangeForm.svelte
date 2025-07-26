<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm, defineSchema } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const schema = defineSchema({
    content_id: {
      type: 'content_pick',
      label: 'Content',
    },
    range: {
      type: 'range',
      label: 'Range',
      props: {
        min: 0,
        max: 100,
      },
    },
  });
  const form = createForm(schema);
  type FormData = CommonSchemaToDataType<typeof schema>;

  async function handleSubmit(data: FormData) {
    add_rule({
      type: 'Range',
      data: {
        content_id: data.content_id,
        range_start: data.range[0],
        range_end: data.range[1],
      },
    });
  }
</script>

<CommonForm {form} {schema} handle={handleSubmit} />
