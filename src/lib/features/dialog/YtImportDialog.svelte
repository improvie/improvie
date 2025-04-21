<script lang='ts'>
  import type { CreateContentResponse } from '$bindings/item/dto';
  import type { YtPlaylistDownloadState, YtVideoDownloadState } from '$bindings/yt';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Progress } from '$lib/components/ui/progress/index.js';
  import { toAppError } from '$lib/error';
  import { Logger } from '$lib/logger';
  import { update_content } from '$lib/stores/items/content';
  import { invoke } from '@tauri-apps/api/core';
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

  let downloading = $state(false);

  let download_state = $state<YtPlaylistDownloadState[]>([{
    index: 0,
    state: {
      downloaded_mb: 0n,
      total_mb: 0n,
      percentage: 0,
    },
  }]);

  listen<YtVideoDownloadState>('yt-download-progress-video', (event) => {
    download_state = [{
      index: 0,
      state: event.payload,
    }];
  });

  listen<YtPlaylistDownloadState>('yt-download-progress-playlist', (event) => {
    const { index, state } = event.payload;
    const existingState = download_state.find(s => s.index === index);

    if (existingState) {
      existingState.state = state;
    }
    else {
      download_state.push({
        index,
        state,
      });
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const result = await validateForm();

    if (!result.valid) {
      return;
    }

    const url = result.data.url;

    downloading = true;
    try {
      Logger.info(`Downloading video id: ${url}`);
      const res = await invoke<CreateContentResponse[]>('import_youtube_url', {
        parentFolderId: parent_folder_id,
        url,
      });
      for (const item of res) {
        update_content(item);
      }
      open = false;
    }
    catch (e) {
      const app_error = toAppError(e);
      if (app_error.kind === 'YtInvalidUrl') {
        toast('Invalid Youtube URL');
      }
      else {
        toast('Video download failed');
        Logger.app_error('Error importing youtube video', app_error);
      }
    }
    downloading = false;
    download_state = [{
      index: 0,
      state: {
        downloaded_mb: 0n,
        total_mb: 0n,
        percentage: 0,
      },
    }];
  }

// TODO: fix style on playlsit, and write test

</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl'>
    {#if downloading}
      <Dialog.Header>
        <Dialog.Title>Downloading...</Dialog.Title>
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        {#each download_state as { index: _, state }}
          <div class='flex flex-col items-center justify-between '>
            <p>Downloaded: {state.downloaded_mb}MB / {state.total_mb}MB</p>
            {#if state.total_mb === 0n}
              <p class='text-muted-foreground'>Fetching video info. Please wait.</p>
            {/if}
            <p>{state.percentage}%</p>
            <Progress
              value={state.percentage}
              max={100}
              class='w-full'
            />
          </div>
        {/each}
      </div>
    {:else}
      <form method='POST' use:enhance onsubmit={handleSubmit}>
        <Dialog.Header>
          <Dialog.Title>Import Youtube Video Or Playlist</Dialog.Title>
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
