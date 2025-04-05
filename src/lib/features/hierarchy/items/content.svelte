<script lang='ts'>
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { contents, delete_content, update_content_name } from '$lib/stores/items/content';
  import { current_track_id } from '$lib/stores/track';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { ImageOffIcon } from 'lucide-svelte';

  let { content_id, rename_data = $bindable() }: {
    content_id: string;
    rename_data: { now_name: string; update_fn: (name: string) => void } | undefined;
  } = $props();

  const content = $derived.by(() => $contents.get(content_id));

  function dblclick() {
    if (content !== undefined) {
      $current_track_id = content_id;
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
    <ContextMenu.Trigger>
      <Card.Root class='p-3 h-full select-none' ondblclick={() => dblclick()}>
        <div class='h-60 md:h-40 flex items-center justify-center'>
          {#if thumbnail_path}
            <img
              src={thumbnail_path}
              alt='Thumbnail not found.'
              class='h-full w-auto object-contain'
            />
          {:else}
            <ImageOffIcon class='h-full w-auto' />
          {/if}
        </div>
        <p class='line-clamp-3 select-text'>{content.title}</p>
      </Card.Root>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>Rename</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}><p class='text-destructive'>Remove</p></ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
