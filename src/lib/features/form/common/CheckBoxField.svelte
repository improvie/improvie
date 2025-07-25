<script lang='ts' module>
  import { z } from 'zod';

  export const CheckBoxFormProps = z.object({
    default: z.boolean().optional(),
  });

  export const CheckBoxFormSchema = z.object({
    type: z.literal('checkbox'),
    label: z.string(),
    props: CheckBoxFormProps.optional(),
  });
</script>

<script lang='ts'>
  import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';
  import * as Form from '$lib/components/ui/form/index.js';

  let {
    value = $bindable(),
    label,
  }: {
    value: boolean;
    label: string;
  } = $props();

</script>

<Form.Control>
  {#snippet children()}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <Checkbox
        class='col-span-5'
        bind:checked={value}
      />
    </div>
  {/snippet}
</Form.Control>
