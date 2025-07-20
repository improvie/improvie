<script lang='ts'>
  import type { ContentKind } from '$bindings/constants';
  import type { CreateContentRequest } from '$bindings/item/command';
  import {
    action_select_content_dialog,
    action_select_thumbnail_dialog,
  } from '$lib/action/dialog/file';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import FormError from '$lib/features/form/FormError.svelte';
  import { Logger } from '$lib/logger';
  import { current_folder_ids } from '$lib/stores/items';
  import { create_content } from '$lib/stores/items/content';
  import { t } from '$lib/translations/translations';
  import { toast } from 'svelte-sonner';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';

  let { open = $bindable() }: { open: boolean } = $props();

  let content_kind: ContentKind | undefined = $state();

  const formSchema = z.object({
    title: z.string().nonempty($t('common.form.no_title')),
    description: z.string().optional(),
    content: z.string().nonempty($t('common.form.no_content')),
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

    if (!result.valid || content_kind === undefined) {
      return;
    }

    const req: CreateContentRequest = {
      title: $formData.title,
      description: $formData.description ?? null,
      content_path: $formData.content,
      thumbnail_path: $formData.thumbnail ?? null,
      parent_folder_id: $current_folder_ids[$current_folder_ids.length - 1],
      kind: content_kind,
    };

    try {
      await create_content(req);
      open = false;
    }
    catch (e) {
      Logger.error('Failed to create content:', e);
      toast.error($t('common.form.error.create_content'));
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
                  placeholder={$t('common.form.content.auto_title')}
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

        <Form.Field {form} name='content'>
          <Form.Control>
            <div class='grid grid-cols-5 items-center gap-4'>
              <Form.Label class='text-right'>Content</Form.Label>
              <Button
                variant='outline'
                class='col-span-4'
                onclick={async () => {
                  const res = await action_select_content_dialog();
                  if (!res) {
                    $formData.content = '';
                    content_kind = undefined;
                  }
                  else {
                    $formData.content = res.path;
                    content_kind = res.kind;

                    if (!$formData.title) {
                      $formData.title = res.name;
                    }
                  }
                }}
              >
                {$formData.content ? $formData.content : 'Select content'}
              </Button>
            </div>
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
