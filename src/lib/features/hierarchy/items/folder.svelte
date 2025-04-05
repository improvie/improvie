<script lang='ts'>
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { current_folder_ids } from '$lib/stores/items';
  import { delete_folder, folders, update_folder_name } from '$lib/stores/items/folder';
  import { FolderIcon } from 'lucide-svelte';

  let { folder_id, rename_data = $bindable() }: { folder_id: string; rename_data: { now_name: string; update_fn: (name: string) => void } | undefined } = $props();

  const folder = $derived.by(() => $folders.get(folder_id));

  function dblclick() {
    current_folder_ids.update((v) => {
      v.push(folder_id);
      return v;
    });
  }

  function rename() {
    rename_data = {
      now_name: folder!.title,
      update_fn: (name: string) => {
        if (folder !== undefined) {
          update_folder_name(folder.id, name);
          folders.update((v) => {
            v.set(folder.id, { ...folder, title: name });
            return v;
          });
        }
      },
    };
  }

  function delete_item() {
    delete_folder(folder_id);
  }

</script>

{#if folder !== undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger>
      <Card.Root class='p-3 h-full' ondblclick={() => dblclick()}>
        <div class='w-full flex items-center justify-center'>
          <FolderIcon class='w-full h-auto' />
        </div>
        <p class='line-clamp-3'>{folder.title}</p>
      </Card.Root>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>Rename</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}><p class='text-destructive'>Remove</p></ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
