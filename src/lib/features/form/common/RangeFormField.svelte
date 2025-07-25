<script lang='ts' module>
  import { z } from 'zod';

  export const RangeFormProps = z.object({
    min: z.number().int().nonnegative().optional(),
    max: z.number().int().nonnegative().optional(),
    step: z.number().int().nonnegative().optional(),
    default: z.tuple([
      z.number().int().nonnegative(),
      z.number().int().nonnegative(),
    ]).optional(),
  });

  export const RangeFormSchema = z.object({
    type: z.literal('range'),
    label: z.string(),
    props: RangeFormProps.optional(),
  });
</script>

<script lang='ts'>
  import * as Form from '$lib/components/ui/form/index.js';
  import { Slider } from '$lib/components/ui/slider/index.js';

  let {
    value = $bindable(),
    label,
    props = {},
  }: {
    value: [number, number];
    label: string;
    props?: z.infer<typeof RangeFormProps>;
  } = $props();

  const { max = 100, min = 0, step = 1 } = props;

</script>

<Form.Control>
  {#snippet children()}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <Slider type='multiple' bind:value max={max} min={min} step={step} />
    </div>
  {/snippet}
</Form.Control>
