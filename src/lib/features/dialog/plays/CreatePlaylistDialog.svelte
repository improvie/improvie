<script lang='ts'>
  import type { CreatePlaylistDto } from '$bindings/play/dto';
  import {
    action_select_thumbnail_dialog,
  } from '$lib/action/dialog/file';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import FormError from '$lib/features/form/FormError.svelte';
  import { current_play_folder_ids } from '$lib/stores/plays';
  import { create_playlist } from '$lib/stores/plays/playlist';
  import { t } from '$lib/translations/translations';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';

  let { open = $bindable() }: { open: boolean } = $props();

  const formSchema = z.object({
    title: z.string().nonempty($t('common.form.no_title')),
    description: z.string().optional(),
    thumbnail: z.string().optional(),
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

    const req: CreatePlaylistDto = {
      title: $formData.title,
      description: $formData.description ?? null,
      thumbnail_path: $formData.thumbnail ?? null,
      parent_folder_id: $current_play_folder_ids[$current_play_folder_ids.length - 1],
    };

    try {
      await create_playlist(req);
      open = false;
    }
    catch (e) {
      console.error(e);
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl'>
    <form method='POST' use:enhance onsubmit={handleSubmit}>
      <Dialog.Header>
        <Dialog.Title>Add Item</Dialog.Title>
        <Dialog.Description>Import audio or video</Dialog.Description>
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        <Form.Field {form} name='title'>
          <Form.Control>
            {#snippet children({ props })}
              <div class='grid grid-cols-5 items-center gap-4'>
                <Form.Label class='text-right'>Title</Form.Label>
                <Input
                  class='col-span-4'
                  placeholder={$t('common.form.no_title')}
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

        <Form.Field {form} name='thumbnail'>
          <Form.Control>
            <div class='grid grid-cols-5 items-center gap-4'>
              <Form.Label class='text-right'>Thumbnail</Form.Label>
              <Button
                variant='outline'
                class='col-span-4'
                onclick={async () => {
                  $formData.thumbnail = await action_select_thumbnail_dialog();
                }}
              >
                <span class='text-muted-foreground'>
                  {$formData.thumbnail ? $formData.thumbnail : 'Select Thumbnail (Option)'}
                </span>
              </Button>
            </div>
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
