<script lang='ts'>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Progress } from '$lib/components/ui/progress/index.js';
  import { toAppError } from '$lib/error';
  import { Logger } from '$lib/logger';
  import { import_youtube_video } from '$lib/stores/items/content';
  import { listen } from '@tauri-apps/api/event';
  import { toast } from 'svelte-sonner';
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

  interface DownloadState {
    downloaded_mb: number;
    total_mb: number;
    percentage: number;
  }

  let downloading = $state(false);

  let download_state = $state<DownloadState>({
    downloaded_mb: 0,
    total_mb: 0,
    percentage: 0,
  });

  listen<DownloadState>('yt-download-progress', (event) => {
    download_state = event.payload;
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();

    if (!result.valid) {
      return;
    }

    downloading = true;
    try {
      await import_youtube_video(parent_folder_id, $formData.url);
      open = false;
    }
    catch (e) {
      const app_error = toAppError(e);
      switch (app_error.kind) {
        case 'YtLoading':
          toast('Please wait while loaded system');
          break;
        case 'YtErrored':
          toast('Video download failed');
          break;
        default:
          toast('Video download failed');
          Logger.app_error('Error importing youtube video', app_error);
          break;
      }
    }
    downloading = false;
    download_state = {
      downloaded_mb: 0,
      total_mb: 0,
      percentage: 0,
    };
  }

</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl'>
    {#if downloading}
      <Dialog.Header>
        <Dialog.Title>Downloading...</Dialog.Title>
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        <div class='flex items-center justify-between'>
          <p>Downloaded: {download_state.downloaded_mb}MB / {download_state.total_mb}MB</p>
          {#if download_state.total_mb === 0}
            <p class='text-muted-foreground'>Fetching video info. Please wait.</p>
          {/if}
          <p>{download_state.percentage}%</p>
        </div>
        <Progress
          value={download_state.percentage}
          max={100}
          class='w-full'
        />
      </div>
    {:else}
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
    {/if}
  </Dialog.Content>
</Dialog.Root>
