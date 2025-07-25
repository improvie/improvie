<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/rule';
  import { action_update_rules } from '$lib/action/rules';
  import FilledIcon from '$lib/components/FilledIcon.svelte';
  import IconButton from '$lib/components/IconButton.svelte';
  import IconText from '$lib/components/IconText.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import Button from '$lib/components/ui/button/button.svelte';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { contents } from '$lib/stores/items/content';
  import { addFavoritePlaylist, favoritePlaylists, removeFavoritePlaylist } from '$lib/stores/plays/favorite';
  import { get_playlist_thumbnail_path } from '$lib/stores/plays/playlist';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { EllipsisVerticalIcon, ListPlusIcon, PlayIcon, ShuffleIcon, StarIcon } from '@lucide/svelte';

  let { playlist = $bindable(), rules: prop_rules }: { playlist: Playlist; rules: RuleType[] } = $props();
  let rules = $state(prop_rules);
  const isFavorite = $derived.by(() => {
    return $favoritePlaylists.includes(playlist.id);
  });
  const playlist_thumbnail_path: Promise<string | undefined> = $derived.by(async () => {
    await action_update_rules(playlist.id, rules);

    return await get_playlist_thumbnail_path(playlist, contents);
  });
  function add_rule(new_rule: RuleType) {
    rules.push(new_rule);
  }
  let open = $state(false);
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<div class='flex flex-col lg:flex-row h-dvh w-full'>
  <div class='h-full mx-auto w-4/5 sm:w-3/5 md:w-2/5 lg:pl-6'>
    {#await playlist_thumbnail_path}
      <ImageLoader loading src={null} />
    {:then src}
      <ImageLoader local src={src} />
    {:catch}
      <ImageLoader src={null} />
    {/await}
    <div class='flex justify-center items-center gap-3 p-3 w-full'>
      <IconButton onclick={() => {
        if (isFavorite) {
          removeFavoritePlaylist(playlist.id);
        }
        else {
          addFavoritePlaylist(playlist.id);
        }
      }}>
        <FilledIcon icon={StarIcon} filled={isFavorite} />
        {#snippet content()}
          <p>{isFavorite ? 'Remove from Favorites' : 'Add to Favorites'}</p>
        {/snippet}
      </IconButton>
      <IconButton variant='default' class='size-12' onclick={() => {
        tracker.set_rules_by_type(playlist.id);
      }}>
        <FilledIcon icon={PlayIcon} filled class='size-8' />
        {#snippet content()}
          <p>Play</p>
        {/snippet}
      </IconButton>
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          <IconButton>
            <EllipsisVerticalIcon />
            {#snippet content()}
              <p>More</p>
            {/snippet}
          </IconButton>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content>
          <DropdownMenu.Item>
            <Button variant='ghost' size='sm' class='p-0' onclick={() => {
              tracker.set_rules_by_type_shuffle(playlist.id);
            }}>
              <IconText icon={ShuffleIcon} text='Shuffle Play' />
            </Button>
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </div>
  </div>
  <ScrollArea orientation='both' class='w-full h-dvh relative z-0'>
    <ContextMenu.Root>
      <ContextMenu.Trigger>
        <div class='w-full flex flex-col gap-6 p-6'>
          {#each rules as _, i}
            <RuleNode bind:rule={rules[i]} remove_rule={() => {
              rules = rules.filter((_, j) => i !== j);
            }} />
          {:else}
            <div class='flex items-center justify-center w-full h-full'>
              <p class='text-muted-foreground'>No rules. Open the context menu to add one.</p>
            </div>
          {/each}
        </div>
        <div class='min-h-80'></div>
      </ContextMenu.Trigger>
      <ContextMenu.Content>
        <ContextMenu.Item onclick={() => {
          open = true;
        }}>
          <IconText icon={ListPlusIcon} text='Add Rule' />
        </ContextMenu.Item>
      </ContextMenu.Content>
    </ContextMenu.Root>
  </ScrollArea>
</div>
