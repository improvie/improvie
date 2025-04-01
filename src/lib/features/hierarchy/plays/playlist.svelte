<script lang='ts'>
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import { select_playlist } from '$lib/stores/plays';
  import { delete_playlist, playlists, update_playlist_name } from '$lib/stores/plays/playlist';
  import { DateTimeFormat } from '$lib/utils';
  import { ListMusicIcon } from 'lucide-svelte';

  let { playlist_id, rename_data = $bindable() }: {
    playlist_id: string;
    rename_data: { now_name: string; update_fn: (name: string) => void } | undefined;
  } = $props();

  const playlist = $derived($playlists.get(playlist_id));

  function rename() {
    rename_data = {
      now_name: playlist!.title,
      update_fn: (name: string) => {
        if (playlist !== undefined) {
          playlist.title = name;
          update_playlist_name(playlist.id, name);
        }
      },
    };
  }

  function delete_item() {
    delete_playlist(playlist_id);
  }

</script>

{#if playlist !== undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger class='contents'>
      <Table.Row ondblclick={() => {
        select_playlist(playlist.id);
      }}>
        <Table.Cell><ListMusicIcon /></Table.Cell>

        <Table.Cell>{playlist.title}</Table.Cell>
        <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, playlist.created_at)}</Table.Cell>
      </Table.Row>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>Rename</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}><p class='text-destructive'>Remove</p></ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
