<script lang='ts' module>
  import type { CommonFieldSchema } from './CommonFormSchema.svelte';

  export type StringFormProps = {
    default?: boolean;
    minLength?: number;
    maxLength?: number;
  };

  export type StringFormSchema = CommonFieldSchema<'string', StringFormProps>;
</script>

<script lang='ts'>
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input';

  let {
    value = $bindable(),
    label,
  }: {
    value: string;
    label: string;
  } = $props();

</script>

<Form.Control>
  {#snippet children(props)}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <Input
        class='col-span-5'
        bind:value={value}
        type='number'
        {...props}
      />
    </div>
  {/snippet}
</Form.Control>
