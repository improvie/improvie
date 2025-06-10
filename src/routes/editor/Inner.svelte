<script module>
  export { default as PlaylistInner } from './Inner.svelte';
</script>

<script lang='ts'>
  import type { Content } from '$bindings/item';

  import type { Playlist } from '$bindings/play';
  import type { RuleType } from '$bindings/rule';
  import { actinn_get_first_rule_format, action_update_rules } from '$lib/action/rules';
  import IconText from '$lib/components/IconText.svelte';
  import ImageLoader from '$lib/components/ImageLoader.svelte';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import CreateRuleDialog from '$lib/features/dialog/rules/CreateRuleDialog.svelte';
  import { RuleNode } from '$lib/features/hierarchy/rules';
  import { contents } from '$lib/stores/items/content';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { cn } from '$lib/utils';
  import { ListPlusIcon } from '@lucide/svelte';

  let { playlist = $bindable(), rules: prop_rules }: { playlist: Playlist; rules: RuleType[] } = $props();
  let rules = $state(prop_rules);
  const playlist_thumbnail_path: Promise<string | undefined> = $derived.by(async () => {
    if (playlist.thumbnail_path) {
      return playlist.thumbnail_path;
    }
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
  $effect(() => {
    action_update_rules(playlist.id, rules);
  });
  function add_rule(new_rule: RuleType) {
    rules.push(new_rule);
  }
  let open = $state(false);
</script>

<CreateRuleDialog add_rule={add_rule} bind:open />

<div class='flex flex-col md:flex-row h-full w-full pl-6'>
  <div class='h-full w-1/3'>
    {#await playlist_thumbnail_path}
      <ImageLoader loading src={null} />
    {:then src}
      <ImageLoader local src={src} />
    {:catch}
      <ImageLoader src={null} />
    {/await}
    <div class='flex flex-col gap-2 mt-2'>
      <h1 class='text-2xl text-center font-bold line-clamp-2'>{playlist.title}</h1>
      <p class='text-muted-foreground line-clamp-3'>{playlist.description}</p>
    </div>
  </div>
  <ScrollArea orientation='both' class={cn('w-full h-dvh relative z-0', open && 'sm:w-2/3')}>
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
