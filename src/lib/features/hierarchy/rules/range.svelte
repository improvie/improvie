<script lang='ts'>
  import type { RangeRule } from '$bindings/Rule';
  import RemoveElement from '$lib/components/element/RemoveElement.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { contents } from '$lib/stores/items/content';
  import { ChevronsUpIcon } from '@lucide/svelte';

  let {
    rule = $bindable(),
    remove_rule,
  }: {
    rule: RangeRule;
    remove_rule: () => void;
  } = $props();
  const content = $derived.by(() => {
    return contents.get(rule.content_id);
  });
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible' oncontextmenu={e => e.stopPropagation()}>
    <ChevronsUpIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <Card.Root class='min-w-48 sm:min-w-80'>
      <Card.Content>
        <div class='flex'>
          <p>{content?.title || 'Loading...'}</p>
          <Separator orientation='vertical' class='mx-1' />
          <p>{rule.range_start}s - {rule.range_end}s</p>
        </div>
      </Card.Content>
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content alignOffset={-20}>
    <ContextMenu.Item onclick={remove_rule}>
      <RemoveElement />
    </ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
