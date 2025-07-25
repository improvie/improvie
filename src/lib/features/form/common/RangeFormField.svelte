<script lang='ts' module>
  import type { CommonFieldSchema } from './CommonFormSchema.svelte';

  export type RangeFormProps = {
    min?: number;
    max?: number;
    step?: number;
    default?: [number, number];
  };

  export type RangeFormSchema = CommonFieldSchema<'range', RangeFormProps>;
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
    props?: RangeFormProps;
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
