<script lang='ts'>
  import {
    Button,
  } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { import_youtube_video } from '$lib/stores/items/content';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';
  import FormError from '../form/FormError.svelte';

  let {
    parent_folder_id,
    open = $bindable(),
  }: {
    parent_folder_id: string;
    open: boolean;
  } = $props();

  const formSchema = z.object({
    url: z.string().url(),
  });

  const form = superForm(defaults(zod(formSchema)), {
    SPA: true,
    validators: zod(formSchema),
    resetForm: false,
  });

  const { form: formData, enhance, validateForm } = form;

  $effect(() => {
    if (!open) {
      form.reset();
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();

    if (!result.valid) {
      return;
    }

    try {
      await import_youtube_video(parent_folder_id, $formData.url);
    }
    catch (e) {
      console.error(e);
    }

    open = false;
  }

</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-[425px]'>
    <form method='POST' use:enhance onsubmit={handleSubmit}>
      <Dialog.Header>
        <Dialog.Title>Import Youtube Video</Dialog.Title>
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        <Form.Field {form} name='url'>
          <Form.Control>
            {#snippet children({ props })}
              <div class='grid grid-cols-5 items-center gap-4'>
                <Form.Label class='text-right'>Url</Form.Label>
                <Input
                  class='col-span-4'
                  {...props}
                  bind:value={$formData.url}
                  spellcheck='false'
                />
              </div>
            {/snippet}
          </Form.Control>
        </Form.Field>
      </div>
      <Dialog.Footer>
        <FormError {form} />
        <Button type='submit'>Import</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
