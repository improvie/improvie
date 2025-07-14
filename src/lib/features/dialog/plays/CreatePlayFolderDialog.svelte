<script lang='ts'>
  import type { CreatePlayFolderDto } from '$bindings/play/dto';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import FormError from '$lib/features/form/FormError.svelte';
  import { Logger } from '$lib/logger';
  import { current_play_folder_ids } from '$lib/stores/plays';
  import { create_play_folder } from '$lib/stores/plays/folder';
  import { t } from '$lib/translations/translations';
  import { toast } from 'svelte-sonner';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';

  let { open = $bindable() }: { open: boolean } = $props();

  const formSchema = z.object({
    title: z.string().nonempty($t('common.form.no_title')),
    description: z.string().optional(),
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

    const req: CreatePlayFolderDto = {
      title: $formData.title,
      description: $formData.description ?? null,
      parent_folder_id: $current_play_folder_ids[$current_play_folder_ids.length - 1],
    };

    try {
      await create_play_folder(req);
      open = false;
    }
    catch (e) {
      Logger.error('Failed to create play folder:', e);
      toast.error('Failed to create play folder. Please try again.');
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl'>
    <form method='POST' use:enhance onsubmit={handleSubmit}>
      <Dialog.Header>
        <Dialog.Title>Add Folder</Dialog.Title>
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        <Form.Field {form} name='title'>
          <Form.Control>
            {#snippet children({ props })}
              <div class='grid grid-cols-5 items-center gap-4'>
                <Form.Label class='text-right'>Title</Form.Label>
                <Input
                  class='col-span-4'
                  {...props}
                  bind:value={$formData.title}
                  spellcheck='false'
                />
              </div>
            {/snippet}
          </Form.Control>
        </Form.Field>

        <Form.Field {form} name='description'>
          <Form.Control>
            {#snippet children({ props })}
              <div class='grid grid-cols-5 items-center gap-4'>
                <Form.Label class='text-right'>Description</Form.Label>
                <Input
                  class='col-span-4'
                  placeholder='Option'
                  {...props}
                  bind:value={$formData.description}
                  spellcheck='false'
                />
              </div>
            {/snippet}
          </Form.Control>
        </Form.Field>
      </div>
      <Dialog.Footer class='flex-col items-center'>
        <FormError {form} />
        <Button type='submit'>Submit</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
