<script lang='ts'>
  import type { ContentRule } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
  import { contents } from '$lib/stores/items/content';
  import { HeadphonesIcon } from 'lucide-svelte';

  let { rule = $bindable(), remove_rule }: { rule: ContentRule; remove_rule: () => void } = $props();
  const content = $derived.by(() => {
    return $contents.get(rule.content_id);
  });
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger>
    <Card.Root class='min-w-80'>
      <Card.Content>
        <div class='flex'>
          <HeadphonesIcon />
          <p>{content?.title || 'Loading...'}</p>
        </div>
      </Card.Content>
    </Card.Root>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item onclick={remove_rule}><p class='text-destructive'>Remove</p></ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
