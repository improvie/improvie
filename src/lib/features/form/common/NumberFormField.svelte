<script lang='ts' module>
  import { z } from 'zod';

  export const NumberFormProps = z.object({
    min: z.number().int().optional(),
    max: z.number().int().optional(),
  });

  export const UintFormSchema = z.object({
    type: z.literal('uint'),
    label: z.string(),
    props: NumberFormProps.optional(),
  });

  export const IntFormSchema = z.object({
    type: z.literal('int'),
    label: z.string(),
    props: NumberFormProps.optional(),
  });
</script>

<script lang='ts'>
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input';

  let {
    value = $bindable(),
    label,
  }: {
    value: number;
    label: string;
  } = $props();

</script>

<Form.Control>
  {#snippet children({ props })}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <Input class='col-span-5' bind:value={value} type='number' {...props} />
    </div>
  {/snippet}
</Form.Control>
