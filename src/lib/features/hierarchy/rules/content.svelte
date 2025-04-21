<script lang='ts'>
  import type { ContentRule } from '$bindings/Rule';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { contents } from '$lib/stores/items/content';
  import { HeadphonesIcon } from '@lucide/svelte';

  let { rule = $bindable(), remove_rule }: { rule: ContentRule; remove_rule: () => void } = $props();
  const content = $derived.by(() => {
    return $contents.get(rule.content_id);
  });
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger class='relative overflow-visible'>
    <HeadphonesIcon class='absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2' />
    <Card.Root class='min-w-80'>
      <div class='flex p-2'>
        <p class='line-clamp-2 select-text'>{content?.title || 'Loading...'}</p>
      </div>
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={remove_rule}><p class='text-destructive'>Remove</p></ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
