<script lang='ts'>
  import type { CreateContentResponse } from '$bindings/item/dto';
  import type { YtPlaylistDownloadState, YtVideoDownloadState } from '$bindings/yt';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Form from '$lib/components/ui/form/index.js';
  import { Input } from '$lib/components/ui/input/index.js';
  import { Progress } from '$lib/components/ui/progress/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
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
  let download_type = $state<'video' | 'playlist'>('video');

  const yt_default_state: YtPlaylistDownloadState = {
    index: 0,
    state: {
      downloaded_mb: 0n,
      total_mb: 0n,
      percentage: 0,
    },
  };

  let download_state = $state<YtPlaylistDownloadState[]>([structuredClone(yt_default_state)]);

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
      if (download_type === 'video') {
        const item = await invoke<CreateContentResponse>('import_youtube_video', {
          parentFolderId: parent_folder_id,
          videoUrlOrId: url,
        });
        update_content(item);
      }
      else if (download_type === 'playlist') {
        const res = await invoke<CreateContentResponse[]>('import_youtube_playlist', {
          parentFolderId: parent_folder_id,
          playlistUrl: url,
        });
        for (const item of res) {
          update_content(item);
        }
      }
      open = false;
    }
    catch (e) {
      const app_error = toAppError(e);
      if (app_error.kind === 'YtInvalidUrl') {
        toast('Invalid Youtube URL');
      }
      else if (app_error.kind === 'YtNotFoundDocumentDir') {
        toast('Please set a document directory by going to settings');
      }
      else {
        toast('Video download failed');
        Logger.app_error('Error importing youtube video', app_error);
      }
    }
    downloading = false;
    download_state = [structuredClone(yt_default_state)];
  }

// TODO: fix style on playlsit, and write test

</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl'>
    {#if downloading}
      <Dialog.Header>
        <Dialog.Title>Downloading...</Dialog.Title>
        {#if download_state[0].state.downloaded_mb === 0n}
          <Dialog.Description>
            <p>Fetching Video Info. Please wait...</p>
          </Dialog.Description>
        {/if}
      </Dialog.Header>
      <div class='grid gap-4 py-4'>
        {#each download_state as { index: _, state }}
          <div>
            <div class='flex items-center justify-between gap-1'>
              <div class='flex flex-wrap gap-1'>
                <p>Downloaded:</p>
                <p>{state.downloaded_mb}MB / {state.total_mb}MB</p>
              </div>
              <p>{state.percentage}%</p>
            </div>
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
