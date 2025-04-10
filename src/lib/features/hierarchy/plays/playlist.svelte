<script lang='ts'>
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { select_playlist } from '$lib/stores/plays';
  import { addFavoritePlaylist, favoritePlaylists, removeFavoritePlaylist } from '$lib/stores/plays/favorite';
  import { delete_playlist, playlists, update_playlist_name } from '$lib/stores/plays/playlist';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let { playlist_id, rename_data = $bindable() }: {
    playlist_id: string;
    rename_data: { now_name: string; update_fn: (name: string) => void } | undefined;
  } = $props();

  const playlist = $derived.by(() => $playlists.get(playlist_id));

  function rename() {
    rename_data = {
      now_name: playlist!.title,
      update_fn: (name: string) => {
        if (playlist !== undefined) {
          update_playlist_name(playlist.id, name);
          playlists.update((v) => {
            v.set(playlist.id, { ...playlist, title: name });
            return v;
          });
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

  let thumbnail_path = $derived.by(() => {
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
    <ContextMenu.Trigger>
      <Card.Root class='z-20 p-3 h-full select-none' ondblclick={() => dblclick()}>
        <div class='flex items-center justify-center'>
          <ImageLoader
            bind:src={thumbnail_path}
          />
        </div>
        <p class='line-clamp-3 select-text'>{playlist.title}</p>
      </Card.Root>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>Rename</ContextMenu.Item>
      {#if is_favorite}
        <ContextMenu.Item onclick={() => removeFavoritePlaylist(playlist.id)}><p class='text-destructive'>Unfavorite</p></ContextMenu.Item>
      {:else}
        <ContextMenu.Item onclick={() => addFavoritePlaylist(playlist.id)}>Favorite</ContextMenu.Item>
      {/if}
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}><p class='text-destructive'>Remove</p></ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
