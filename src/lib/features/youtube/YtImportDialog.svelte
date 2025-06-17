<script lang='ts'>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import { defaults, superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';
  import FormError from '../form/FormError.svelte';
  import YtImportPlaylist from './YtImportPlaylist.svelte';
  import YtImportVideo from './YtImportVideo.svelte';

  let {
    parent_folder_id,
    open = $bindable(),
  }: {
    parent_folder_id: string;
    open: boolean;
  } = $props();

  let download_type = $state<'video' | 'playlist'>('video');

  const formSchema = z.object({
    url: z.string().nonempty().url(),
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

  let start_processing = $state(false);

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();

    if (!result.valid) {
      return;
    }

    start_processing = true;
  }

</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl'>
    {#if start_processing}
      {#if download_type === 'video'}
        <YtImportVideo
          parent_folder_id={parent_folder_id}
          url={$formData.url}
        />
      {:else if download_type === 'playlist'}
        <YtImportPlaylist
          parent_folder_id={parent_folder_id}
          url={$formData.url}
        />
      {/if}
    {:else}
      <form method='POST' use:enhance onsubmit={handleSubmit}>
        <Dialog.Header>
          <Dialog.Title>Import Youtube</Dialog.Title>
        </Dialog.Header>

        <Tabs.Root bind:value={download_type} class='mt-2 md:-mt-6 flex items-center'>
          <Tabs.List>
            <Tabs.Trigger value='video'>Video</Tabs.Trigger>
            <Tabs.Trigger value='playlist'>Playlist</Tabs.Trigger>
          </Tabs.List>
        </Tabs.Root>

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
    {/if}
  </Dialog.Content>
</Dialog.Root>
