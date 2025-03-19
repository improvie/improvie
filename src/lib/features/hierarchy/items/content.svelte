<script lang='ts'>
  import type { Content } from '$lib/types/item';
  import * as Table from '$lib/components/ui/table/index.js';
  import { contents } from '$lib/stores/items/content';
  import { DateTimeFormat } from '$lib/utils';
  import { FileMusicIcon, FileQuestionIcon, FileVideoIcon } from 'lucide-svelte';

  let { content_id, audio_inspector_content = $bindable() }: {
    content_id: string;
    audio_inspector_content: Content | undefined;
  } = $props();

  const content = $derived($contents.get(content_id));

  function dblclick() {
    if (content !== undefined && content.kind === 'Audio') {
      audio_inspector_content = content;
    }
  }
</script>

{#if content !== undefined}
  <Table.Row ondblclick={dblclick}>
    {#if content.kind === 'Audio'}
      <Table.Cell><FileMusicIcon /></Table.Cell>
    {:else if content.kind === 'Video'}
      <Table.Cell><FileVideoIcon /></Table.Cell>
    {/if}

    <Table.Cell>{content.title}</Table.Cell>
    <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, content.created_at)}</Table.Cell>
  </Table.Row>
{:else}
  <Table.Row>
    <Table.Cell><FileQuestionIcon /></Table.Cell>
    <Table.Cell>Loading...</Table.Cell>
    <Table.Cell class='text-right'>...</Table.Cell>
  </Table.Row>
{/if}
