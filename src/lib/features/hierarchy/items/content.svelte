<script lang='ts'>
  import * as Table from '$lib/components/ui/table/index.js';
  import { contents } from '$lib/stores/items/content';
  import { DateTimeFormat } from '$lib/utils';
  import { FileMusicIcon, FileVideoIcon } from 'lucide-svelte';

  const { content_id }: { content_id: string } = $props();

  const content = $derived($contents.get(content_id));
</script>

{#if content !== undefined}
  <Table.Row>
    {#if content.kind === 'Audio'}
      <Table.Cell><FileMusicIcon /></Table.Cell>
    {:else if content.kind === 'Video'}
      <Table.Cell><FileVideoIcon /></Table.Cell>
    {/if}

    <Table.Cell>{content.title}</Table.Cell>
    <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, content.created_at)}</Table.Cell>
  </Table.Row>
{/if}
