<script lang='ts'>
  import type { Content } from '$lib/types/item';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import * as Table from '$lib/components/ui/table/index.js';
  import { contents, delete_content, update_content_name } from '$lib/stores/items/content';
  import { DateTimeFormat } from '$lib/utils';
  import { FileMusicIcon, FileVideoIcon } from 'lucide-svelte';

  let { content_id, audio_inspector_content = $bindable(), rename_data = $bindable() }: {
    content_id: string;
    audio_inspector_content: Content | undefined;
    rename_data: { now_name: string; update_fn: (name: string) => void } | undefined;
  } = $props();

  const content = $derived.by(() => $contents.get(content_id));

  function dblclick() {
    if (content !== undefined && content.kind === 'Audio') {
      audio_inspector_content = content;
    }
  }

  function rename() {
    rename_data = {
      now_name: content!.title,
      update_fn: (name: string) => {
        if (content !== undefined) {
          update_content_name(content.id, name);
          contents.update((v) => {
            v.set(content.id, { ...content, title: name });
            return v;
          });
        }
      },
    };
  }

  function delete_item() {
    delete_content(content_id);
  }

</script>

{#if content !== undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger class='contents'>
      <Table.Row ondblclick={dblclick}>
        {#if content.kind === 'Audio'}
          <Table.Cell><FileMusicIcon /></Table.Cell>
        {:else if content.kind === 'Video'}
          <Table.Cell><FileVideoIcon /></Table.Cell>
        {/if}

        <Table.Cell>{content.title}</Table.Cell>
        <Table.Cell class='text-right'>{DateTimeFormat.format(DateTimeFormat.PlainYmdHms, content.created_at)}</Table.Cell>
      </Table.Row>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>Rename</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}><p class='text-destructive'>Remove</p></ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
