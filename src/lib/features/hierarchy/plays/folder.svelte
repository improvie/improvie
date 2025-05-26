<script lang='ts'>
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import RenameElement from '$lib/components/element/RenameElement.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { current_play_folder_ids } from '$lib/stores/plays';
  import { delete_play_folder, play_folders, update_play_folder_name } from '$lib/stores/plays/folder';
  import { FolderIcon } from '@lucide/svelte';

  let { folder_id, rename_data = $bindable() }: { folder_id: string; rename_data: { now_name: string; update_fn: (name: string) => void } | undefined } = $props();

  const folder = $derived(play_folders.get(folder_id));

  function dblclick() {
    current_play_folder_ids.update((v) => {
      v.push(folder_id);
      return v;
    });
  }

  function rename() {
    rename_data = {
      now_name: folder!.title,
      update_fn: (name: string) => {
        if (folder !== undefined) {
          update_play_folder_name(folder.id, name);
          play_folders.set(folder.id, { ...folder, title: name });
        }
      },
    };
  }

  function delete_item() {
    delete_play_folder(folder_id);
  }

</script>

{#if folder !== undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger class='z-20'>
      <Card.Root class='p-3 h-full' ondblclick={() => dblclick()}>
        <div class='flex items-center justify-center'>
          <FolderIcon class='w-9/16 h-auto aspect-square' />
        </div>
        <p class='line-clamp-3'>{folder.title}</p>
      </Card.Root>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>
        <RenameElement />
      </ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}>
        <RemoveElement />
      </ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
