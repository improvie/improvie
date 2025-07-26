<script lang='ts' module>
  import type { CommonFieldSchema } from './CommonFormSchema.svelte';

  export type ContentPickFormSchema = CommonFieldSchema<'content_pick', {}>;
</script>

<script lang='ts'>
  import type { PickItem } from '$lib/types/item';
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import ContentPicker from '$lib/features/combobox/ContentPicker.svelte';

  let {
    value = $bindable(),
    label,
  }: {
    value: string;
    label: string;
  } = $props();

  let content: undefined | PickItem = $state();
  let open = $state(false);

</script>

<ContentPicker bind:content={() => content,
  (v) => {
  content = v;
  value = v?.id ?? '';
  }} bind:open />

<Form.Control>
  {#snippet children(props)}
    <div class='grid grid-cols-7 items-center gap-4'>
      <Form.Label class='text-right col-span-2'>{label}</Form.Label>
      <Button
        class='col-span-5 text-wrap'
        variant='outline'
        {...props}
        onclick={() => open = true}
      >
        {content?.hierarchy_name || 'Select content'}
      </Button>
    </div>
  {/snippet}
</Form.Control>
