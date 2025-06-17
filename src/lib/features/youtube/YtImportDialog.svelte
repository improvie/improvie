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

  interface Targets {
    videoId?: string;
    playlistId?: string;
  }

  const formSchema = z.object({
    targets: z.string().nonempty().url().transform<Targets>((value, ctx) => {
      const url = new URL(value);
      const list = url.searchParams.get('list');
      if (download_type === 'playlist') {
        if (list === null || list === '') {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: 'Invalid YouTube URL. It must contain a playlist ID.',
          });
          return {};
        }
        // 'RD' is a prefix for mixlist, require videoId in this case
        if (!list.startsWith('RD')) {
          return { playlistId: list };
        }
      }
      const videoId = url.searchParams.get('v');
      if (videoId === null || videoId === '') {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: 'Invalid YouTube URL. It must contain a video ID.',
        });
      }
      return {
        videoId: videoId ?? undefined,
        playlistId: list ?? undefined,
      };
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
          videoId={$formData.targets.videoId!}
        />
      {:else if download_type === 'playlist'}
        <YtImportPlaylist
          parent_folder_id={parent_folder_id}
          playlistId={$formData.targets.playlistId!}
          videoId={$formData.targets.videoId}
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
          <Form.Field {form} name='targets'>
            <Form.Control>
              {#snippet children({ props })}
                <div class='grid grid-cols-5 items-center gap-4'>
                  <Form.Label class='text-right'>Url</Form.Label>
                  <Input
                    class='col-span-4'
                    {...props}
                    bind:value={$formData.targets}
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
