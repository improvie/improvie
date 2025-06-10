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
  const first_content: Promise<Content | undefined> = $derived.by(async () => {
    const format = await actinn_get_first_rule_format(rules);
    if (format === undefined) {
      return undefined;
    }
    return contents.get(format.content_id);
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

<div class='flex flex-col md:flex-row h-full w-full'>
  <div class='h-full w-1/3'>
    <div class='aspect-video'>
      {#await first_content}
        <div class='w-full h-full flex items-center justify-center'>Loading...</div>
      {:then content}
        {#if content}
          <ImageLoader local bind:src={content.thumbnail_path} bind:alt={content.title} />
        {:else}
          <div class='w-full h-full flex items-center justify-center'>No content available.</div>
        {/if}
      {:catch e}
        {console.error(e)}
        <div class='w-full h-full flex items-center justify-center'>Error loading content.</div>
      {/await}
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
