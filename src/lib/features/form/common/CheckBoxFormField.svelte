<script lang='ts' module>
  import type { CommonFieldSchema } from './CommonFormSchema.svelte';

  export type CheckBoxFormProps = {
    default?: boolean;
    description?: string;
  };

  export type CheckBoxFormSchema = CommonFieldSchema<'checkbox', CheckBoxFormProps>;
</script>

<script lang='ts'>
  import Checkbox from '$lib/components/ui/checkbox/checkbox.svelte';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Label } from '$lib/components/ui/label';

  let {
    value = $bindable(),
    label,
    props,
  }: {
    value: boolean;
    label: string;
    props?: CheckBoxFormProps;
  } = $props();

</script>

<Form.Control>
  {#snippet children()}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <div class='flex items-center gap-3 col-span-5'>
        <Checkbox
          bind:checked={value}
        />
        {#if props?.description}
          <Label class='text-left text-nowrap text-muted-foreground'>{props.description}</Label>
        {/if}
      </div>
    </div>
  {/snippet}
</Form.Control>
