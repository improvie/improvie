<script lang='ts' module>
  import type { CommonFieldSchema } from './CommonFormSchema.svelte';

  export type NumberFormProps = {
    min?: number;
    max?: number;
    description?: string;
  };

  export type UintFormSchema = CommonFieldSchema<'uint', NumberFormProps>;
  export type IntFormSchema = CommonFieldSchema<'int', NumberFormProps>;
</script>

<script lang='ts'>
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';

  let {
    value = $bindable(),
    label,
    props,
  }: {
    value: number;
    label: string;
    props?: NumberFormProps;
  } = $props();

</script>

<Form.Control>
  {#snippet children({ props: inputProps })}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <div class='flex w-full items-center gap-3 col-span-5'>
        <Input bind:value={value} type='number' {...inputProps} />
        {#if props?.description}
          <Label class='text-left text-nowrap text-muted-foreground'>{props.description}</Label>
        {/if}
      </div>
    </div>
  {/snippet}
</Form.Control>
