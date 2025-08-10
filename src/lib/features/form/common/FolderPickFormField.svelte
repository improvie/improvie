<script lang='ts' module>
  import type { CommonFieldSchema } from './CommonFormSchema.svelte';

  export type FolderPickFormSchema = CommonFieldSchema<'folder_pick', {}>;
</script>

<script lang='ts'>
  import { Button } from '$lib/components/ui/button';
  import * as Form from '$lib/components/ui/form/index.js';
  import FolderPicker from '$lib/features/combobox/FolderPicker.svelte';
  import { folders } from '$lib/stores/items/folder';

  let {
    value = $bindable(),
    label,
  }: {
    value: string;
    label: string;
  } = $props();

  let open = $state(false);
  const folder = $derived.by(() => folders.get(value));

</script>

<FolderPicker bind:folder_id={value} bind:open />

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
        {folder?.title || 'Select folder'}
      </Button>
    </div>
  {/snippet}
</Form.Control>
