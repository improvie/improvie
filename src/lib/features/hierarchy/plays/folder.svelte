<script lang='ts'>
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import { current_play_folder_ids } from '$lib/stores/plays';
  import { delete_play_folder, play_folders, update_play_folder_name } from '$lib/stores/plays/folder';
  import { DateTimeFormat } from '$lib/utils';
  import { FolderIcon } from 'lucide-svelte';

  let { folder_id, rename_data = $bindable() }: { folder_id: string; rename_data: { now_name: string; update_fn: (name: string) => void } | undefined } = $props();

  const folder = $derived.by(() => $play_folders.get(folder_id));

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
          play_folders.update((v) => {
            v.set(folder.id, { ...folder, title: name });
            return v;
          });
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
    <ContextMenu.Trigger class='contents'>
      <Table.Row ondblclick={() => dblclick()}>
        <Table.Cell><FolderIcon /></Table.Cell>
        <Table.Cell>{folder.title}</Table.Cell>
        <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, folder.created_at)}</Table.Cell>
      </Table.Row>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>Rename</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}><p class='text-destructive'>Remove</p></ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
