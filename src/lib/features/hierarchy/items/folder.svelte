<script lang='ts'>
  import * as Table from '$lib/components/ui/table/index.js';
  import { current_folder_ids } from '$lib/stores/items';
  import { folders } from '$lib/stores/items/folder';
  import { FolderIcon } from 'lucide-svelte';

  const { folder_id }: { folder_id: string } = $props();

  const folder = $folders.get(folder_id);

  function dblclick() {
    current_folder_ids.update((v) => {
      v.push(folder_id);
      return v;
    });
  }

</script>

{#if folder !== undefined}
  <Table.Row ondblclick={() => dblclick()}>
    <Table.Cell><FolderIcon /></Table.Cell>
    <Table.Cell>{folder.title}</Table.Cell>
    <Table.Cell class='text-right'>{folder.created_at}</Table.Cell>
  </Table.Row>
  <!-- <ContextMenu.Root> -->
  <!--   <ContextMenu.Trigger> -->
  <!--   </ContextMenu.Trigger> -->
  <!--   <ContextMenu.Content> -->
  <!--     <ContextMenu.Item>Rename</ContextMenu.Item> -->
  <!--     <ContextMenu.Separator /> -->
  <!--     <ContextMenu.Item class='text-destructive'>Remove</ContextMenu.Item> -->
  <!--   </ContextMenu.Content> -->
  <!-- </ContextMenu.Root> -->
{/if}
