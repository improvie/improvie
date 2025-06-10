<script lang='ts'>
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import RenameElement from '$lib/components/element/RenameElement.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { contents, delete_content, update_content_name } from '$lib/stores/items/content';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let { content_id, rename_data = $bindable() }: {
    content_id: string;
    rename_data: { now_name: string; update_fn: (name: string) => void } | undefined;
  } = $props();

  const content = $derived(contents.get(content_id));

  function dblclick() {
    if (content !== undefined) {
      tracker.set_single_content(content.id);
    }
  }

  function rename() {
    rename_data = {
      now_name: content!.title,
      update_fn: (name: string) => {
        if (content !== undefined) {
          update_content_name(content.id, name);
          contents.set(content.id, { ...content, title: name });
        }
      },
    };
  }

  function delete_item() {
    delete_content(content_id);
  }

  const thumbnail_path = $derived.by(() => {
    if (content === undefined) {
      return undefined;
    }
    if (!content.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(content.thumbnail_path);
  });

</script>

{#if content !== undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger class='z-20'>
      <Card.Root class='p-3 gap-1' ondblclick={() => dblclick()}>
        <ImageLoader src={thumbnail_path} />
        <p class='line-clamp-3'>{content.title}</p>
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
