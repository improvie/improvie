<script lang='ts'>
  import type { ContentRule } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import { contents } from '$lib/stores/items/content';
  import { HeadphonesIcon } from 'lucide-svelte';

  let { rule = $bindable() }: { rule: ContentRule } = $props();
  const content = $derived.by(() => {
    return $contents.get(rule.content_id);
  });
</script>

<Card.Root class='min-w-80'>
  <Card.Content>
    <div class='flex'>
      <HeadphonesIcon />
      <p>{content?.title || 'Loading...'}</p>
    </div>
  </Card.Content>
</Card.Root>
