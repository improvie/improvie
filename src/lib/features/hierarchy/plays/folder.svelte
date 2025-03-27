<script lang='ts'>
  import * as Table from '$lib/components/ui/table/index.js';
  import { current_play_folder_ids } from '$lib/stores/plays';
  import { play_folders } from '$lib/stores/plays/folder';
  import { DateTimeFormat } from '$lib/utils';
  import { FolderIcon, FolderXIcon } from 'lucide-svelte';

  const { folder_id }: { folder_id: string } = $props();

  const folder = $derived($play_folders.get(folder_id));

  function dblclick() {
    current_play_folder_ids.update((v) => {
      v.push(folder_id);
      return v;
    });
  }

</script>

{#if folder !== undefined}
  <Table.Row ondblclick={() => dblclick()}>
    <Table.Cell><FolderIcon /></Table.Cell>
    <Table.Cell>{folder.title}</Table.Cell>
    <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, folder.created_at)}</Table.Cell>
  </Table.Row>
{:else}
  <Table.Row>
    <Table.Cell><FolderXIcon /></Table.Cell>
    <Table.Cell>Loading...</Table.Cell>
    <Table.Cell class='text-right'>...</Table.Cell>
  </Table.Row>
{/if}
