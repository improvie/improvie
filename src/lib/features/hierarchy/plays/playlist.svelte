<script lang='ts'>
  import * as Table from '$lib/components/ui/table/index.js';
  import { select_playlist } from '$lib/stores/plays';
  import { playlists } from '$lib/stores/plays/playlist';
  import { DateTimeFormat } from '$lib/utils';
  import { ListMusicIcon, ListXIcon } from 'lucide-svelte';

  const { playlist_id }: {
    playlist_id: string;
  } = $props();

  const playlist = $derived($playlists.get(playlist_id));

</script>

{#if playlist !== undefined}
  <Table.Row ondblclick={() => {
    select_playlist(playlist.id);
  }}>
    <Table.Cell><ListMusicIcon /></Table.Cell>

    <Table.Cell>{playlist.title}</Table.Cell>
    <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, playlist.created_at)}</Table.Cell>
  </Table.Row>
{:else}
  <Table.Row>
    <Table.Cell><ListXIcon /></Table.Cell>
    <Table.Cell>Loading...</Table.Cell>
    <Table.Cell class='text-right'>...</Table.Cell>
  </Table.Row>
{/if}
