<script lang='ts'>
  import { actinn_get_first_rule_format, action_get_rules } from '$lib/action/rules';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import RenameElement from '$lib/components/element/RenameElement.svelte';
  import IconText from '$lib/components/IconText.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { contents } from '$lib/stores/items/content';
  import { select_playlist } from '$lib/stores/plays';
  import { addFavoritePlaylist, favoritePlaylists, removeFavoritePlaylist } from '$lib/stores/plays/favorite';
  import { delete_playlist, playlists, update_playlist_name } from '$lib/stores/plays/playlist';
  import { StarIcon, StarOffIcon } from '@lucide/svelte';

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

  const playlist_thumbnail_path: Promise<string | undefined> = $derived.by(async () => {
    if (playlist === undefined) {
      return undefined;
    }
    if (playlist.thumbnail_path) {
      return playlist.thumbnail_path;
    }
    const rules = await action_get_rules(playlist.id);
    const format = await actinn_get_first_rule_format(rules);
    if (format === undefined) {
      return undefined;
    }
    const content = contents.get(format.content_id);
    if (content === undefined) {
      return undefined;
    }
    return content.thumbnail_path ? content.thumbnail_path : undefined;
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
      <Card.Root class='p-3 gap-1' ondblclick={() => dblclick()}>
        {#await playlist_thumbnail_path}
          <ImageLoader loading src={null} />
        {:then src}
          <ImageLoader local src={src} />
        {:catch}
          <ImageLoader src={null} />
        {/await}
        <p class='line-clamp-3'>{playlist.title}</p>
      </Card.Root>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item onclick={rename}>
        <RenameElement />
      </ContextMenu.Item>
      {#if is_favorite}
        <ContextMenu.Item onclick={() => removeFavoritePlaylist(playlist.id)} class='text-destructive'>
          <IconText icon={StarOffIcon} text='Unfavorite' />
        </ContextMenu.Item>
      {:else}
        <ContextMenu.Item onclick={() => addFavoritePlaylist(playlist.id)}>
          <IconText icon={StarIcon} text='Favorite' />
        </ContextMenu.Item>
      {/if}
      <ContextMenu.Separator />
      <ContextMenu.Item onclick={delete_item}>
        <RemoveElement />
      </ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
