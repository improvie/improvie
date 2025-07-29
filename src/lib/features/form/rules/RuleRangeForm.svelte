<script lang='ts' module>
  import type { CommonSchemaToDataType } from '../common/CommonFormSchema.svelte';
  import { defineSchema } from '../common/CommonFormSchema.svelte';

  export const RangeRuleSchema = $state(defineSchema({
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
  export type RangeRuleFormData = CommonSchemaToDataType<typeof RangeRuleSchema>;
</script>

<script lang='ts'>
  import type { RuleType } from '$bindings/rule';
  import { contents } from '$lib/stores/items/content';
  import CommonForm from '../common/CommonForm.svelte';
  import { createForm } from '../common/CommonFormSchema.svelte';

  let { add_rule = $bindable() }: { add_rule: (rule: RuleType) => void } = $props();

  const form = createForm(RangeRuleSchema);

  async function handleSubmit(data: RangeRuleFormData) {
    add_rule({
      type: 'Range',
      data: {
        content_id: data.content_id,
        range_start: data.range[0],
        range_end: data.range[1],
      },
    });
  }

  function handleChange(data: RangeRuleFormData) {
    if (!(data.content_id && RangeRuleSchema.range.props.disabled)) {
      return;
    }
    RangeRuleSchema.range.props.disabled = '';
    const content = contents.get(data.content_id);
    if (!content) {
      return;
    }
    RangeRuleSchema.range.props.min = 0;
    RangeRuleSchema.range.props.max = content.seconds;
    data.range = [0, content.seconds];
    return data;
  }
</script>

<CommonForm {form} schema={RangeRuleSchema} handle={handleSubmit} handleChange={handleChange} />
