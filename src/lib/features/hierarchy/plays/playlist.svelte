<script lang='ts'>
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import RenameElement from '$lib/components/element/RenameElement.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { select_playlist } from '$lib/stores/plays';
  import { addFavoritePlaylist, favoritePlaylists, removeFavoritePlaylist } from '$lib/stores/plays/favorite';
  import { delete_playlist, playlists, update_playlist_name } from '$lib/stores/plays/playlist';
  import { StarIcon, StarOffIcon } from '@lucide/svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let { playlist_id, rename_data = $bindable() }: {
    playlist_id: string;
    rename_data: { now_name: string; update_fn: (name: string) => void } | undefined;
  } = $props();

  const playlist = $derived(playlists.get(playlist_id));

  function rename() {
    rename_data = {
      now_name: playlist!.title,
      update_fn: (name: string) => {
        if (playlist !== undefined) {
          update_playlist_name(playlist.id, name);
          playlists.set(playlist.id, { ...playlist, title: name });
        }
      },
    };
  }

  function delete_item() {
    delete_playlist(playlist_id);
  }

  function dblclick() {
    if (playlist !== undefined) {
      select_playlist(playlist.id);
    }
  }

  const thumbnail_path = $derived.by(() => {
    if (playlist === undefined) {
      return undefined;
    }
    if (!playlist.thumbnail_path) {
      return undefined;
    }
    return convertFileSrc(playlist.thumbnail_path);
  });

  const is_favorite = $derived.by(() => {
    if (playlist === undefined) {
      return false;
    }
    return $favoritePlaylists.includes(playlist.id);
  });

</script>

{#if playlist !== undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger class='z-20'>
      <Card.Root class='p-3 h-full' ondblclick={() => dblclick()}>
        <div class='flex items-center justify-center'>
          <ImageLoader
            src={thumbnail_path}
          />
        </div>
        <p class='line-clamp-3 select-text'>{playlist.title}</p>
      </Card.Root>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>
        <RenameElement />
      </ContextMenu.Item>
      {#if is_favorite}
        <ContextMenu.Item onclick={() => removeFavoritePlaylist(playlist.id)} class='text-destructive'>
          <StarOffIcon class='mr-1 size-4' />Unfavorite
        </ContextMenu.Item>
      {:else}
        <ContextMenu.Item onclick={() => addFavoritePlaylist(playlist.id)}>
          <StarIcon class='mr-1 size-4' />Favorite
        </ContextMenu.Item>
      {/if}
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}>
        <RemoveElement />
      </ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
