<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import { contents } from '$lib/stores/items/content';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm, defineSchema } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const schema = $state(defineSchema({
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
        disabled: '先にコンテンツを選択してください',
      },
    },
  }));
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

  function handleChange(data: FormData) {
    if (!(data.content_id && schema.range.props.disabled)) {
      return;
    }
    schema.range.props.disabled = '';
    const content = contents.get(data.content_id);
    if (!content) {
      return;
    }
    schema.range.props.min = 0;
    schema.range.props.max = content.seconds;
  }
</script>

<CommonForm {form} {schema} handle={handleSubmit} handleChange={handleChange} />
