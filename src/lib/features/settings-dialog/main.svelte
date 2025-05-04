<script lang='ts'>
  import type { AppSettings } from '$bindings/settings/index';
  import { action_select_folder_dialog } from '$lib/action/dialog/folder';
  import { Button } from '$lib/components/ui/button';
  import { Label } from '$lib/components/ui/label/index.js';
  import { FilmIcon } from '@lucide/svelte';

  let { settings = $bindable() }: { settings: AppSettings } = $props();

  async function click() {
    const selected = await action_select_folder_dialog();
    if (selected !== null) {
      settings.document_dir = selected;
    }
  }
</script>

<div class='flex justify-between'>
  <div class='flex text-left items-center gap-2'>
    <FilmIcon class='size-10' />
    <Label>Document Directory</Label>
  </div>
  <Button variant='outline' onclick={() => click()} class='w-full max-w-full break-all'>{settings.document_dir || 'Not set'}</Button>
</div>
