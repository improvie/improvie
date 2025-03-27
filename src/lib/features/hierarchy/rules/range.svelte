<script lang='ts'>
  import type { RangeRule } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import { contents } from '$lib/stores/items/content';
  import { ChevronsUpIcon } from 'lucide-svelte';

  let { rule = $bindable() }: { rule: RangeRule } = $props();
  const content = $derived.by(() => {
    return $contents.get(rule.content_id);
  });
</script>

<Card.Root>
  <Card.Content>
    <ChevronsUpIcon />
    <p>{content?.title || 'Loading...'}</p>
  </Card.Content>
</Card.Root>
