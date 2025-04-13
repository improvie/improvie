<script lang='ts'>
  import type { CreateContentResponse } from '$lib/types/item/create';
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
  import Innertube, { Types } from 'youtubei.js';
  import { z } from 'zod';
  import FormError from '../form/FormError.svelte';

  let {
    parent_folder_id,
    open = $bindable(),
  }: {
    parent_folder_id: string;
    open: boolean;
  } = $props();

  const youtubeUrlRegex
    = /^(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/watch\?v=|youtu\.be\/)([\w-]{11})(?:[&?][\w=]*)*$/;

  const formSchema = z.object({
    url: z.string()
      .regex(youtubeUrlRegex, {
        message: 'Invalid YouTube URL',
      }),
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

    const yt = await Innertube.create({
      generate_session_locally: false,
      enable_session_cache: false,
    });
    const video_id = result.data.url.match(youtubeUrlRegex)![1]!;
    const info = await yt.getInfo(video_id);
    const title = info.basic_info.title;

    const opts: Types.DownloadOptions = {
      quality: '360p',
      type: 'video+audio',
      format: 'mp4',
      range: undefined,
    };

    const format = info.chooseFormat(opts);
    const formatUrl = format.decipher(yt.session.player);
    const videoUrl = `${formatUrl}&cpn=${info.cpn}`;

    downloading = true;
    try {
      const res = await invoke<CreateContentResponse>('import_youtube_video', {
        parentFolderId: parent_folder_id,
        title,
        videoUrl,
      });
      update_content(res);
      open = false;
    }
    catch (e) {
      const app_error = toAppError(e);
      toast('Video download failed');
      Logger.app_error('Error importing youtube video', app_error);
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
