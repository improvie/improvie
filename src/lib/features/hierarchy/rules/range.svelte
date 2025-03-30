<script lang='ts'>
  import type { RangeRule } from '$lib/types/rules';
  import * as Card from '$lib/components/ui/card/index.js';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { contents } from '$lib/stores/items/content';
  import { ChevronsUpIcon } from 'lucide-svelte';

  let { rule = $bindable() }: { rule: RangeRule } = $props();
  const content = $derived.by(() => {
    return $contents.get(rule.content_id);
  });
</script>

<Card.Root class='min-w-80'>
  <Card.Content>
    <div class='flex'>
      <ChevronsUpIcon />
      <p>{content?.title || 'Loading...'}</p>
      <Separator orientation='vertical' class='mx-1' />
      <p>{rule.range_start}s - {rule.range_end}s</p>
    </div>
  </Card.Content>
</Card.Root>
