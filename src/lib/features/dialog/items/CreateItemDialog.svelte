<script lang="ts">
  import {
    action_select_content_dialog,
    action_select_thumbnail_dialog,
  } from '$lib/action/dialog/file';
  import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import FormError from '$lib/features/form/FormError.svelte';
  import { t } from '$lib/translations/translations';
  import { debug } from '@tauri-apps/plugin-log';
  import { ImportIcon } from 'lucide-svelte';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';

  let open = $state(false);

  const formSchema = z.object({
    title: z.string().nonempty($t('common.items.add_content.no_title')),
    description: z.string().optional(),
    content: z.string().nonempty($t('common.items.add_content.no_content')),
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

    debug(`Form data: ${JSON.stringify($formData)}`);
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger class={buttonVariants({ variant: 'outline' })}>
    <ImportIcon class="mr-2 size-4" />Add Item
  </Dialog.Trigger>
  <Dialog.Content>
    <form method="POST" use:enhance onsubmit={handleSubmit}>
      <Dialog.Header>
        <Dialog.Title>Add Item</Dialog.Title>
        <Dialog.Description>Import audio or video</Dialog.Description>
      </Dialog.Header>
      <div class="grid gap-4 py-4">
        <Form.Field {form} name="title">
          <Form.Control>
            {#snippet children({ props })}
              <div class="grid grid-cols-5 items-center gap-4">
                <Form.Label class="text-right">Title</Form.Label>
                <Input
                  class="col-span-4"
                  placeholder={$t('common.items.add_content.auto_title')}
                  {...props}
                  bind:value={$formData.title}
                  spellcheck="false"
                />
              </div>
            {/snippet}
          </Form.Control>
        </Form.Field>

        <Form.Field {form} name="description">
          <Form.Control>
            {#snippet children({ props })}
              <div class="grid grid-cols-5 items-center gap-4">
                <Form.Label class="text-right">Description</Form.Label>
                <Input
                  class="col-span-4"
                  placeholder="Option"
                  {...props}
                  bind:value={$formData.description}
                  spellcheck="false"
                />
              </div>
            {/snippet}
          </Form.Control>
        </Form.Field>

        <Form.Field {form} name="content">
          <Form.Control>
            <div class="grid grid-cols-5 items-center gap-4">
              <Form.Label class="text-right">Content</Form.Label>
              <Button
                variant="outline"
                class="col-span-4"
                onclick={async () => {
                  const res = await action_select_content_dialog();
                  if (!res) {
                    $formData.content = '';
                  } else {
                    $formData.content = res.path;

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

        <Form.Field {form} name="thumbnail">
          <Form.Control>
            <div class="grid grid-cols-5 items-center gap-4">
              <Form.Label class="text-right">Thumbnail</Form.Label>
              <Button
                variant="outline"
                class="col-span-4"
                onclick={async () => {
                  $formData.thumbnail = (await action_select_thumbnail_dialog())?.path;
                }}
              >
                <span class="text-muted-foreground">
                  {$formData.thumbnail ? $formData.thumbnail : 'Select Thumbnail (Option)'}
                </span>
              </Button>
            </div>
          </Form.Control>
        </Form.Field>
      </div>
      <Dialog.Footer class="flex-col items-center">
        <FormError {form} />
        <Button type="submit">Submit</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
