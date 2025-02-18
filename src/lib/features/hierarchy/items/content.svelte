<script lang='ts'>
  import { contents } from '$lib/stores/items/content';
  import { FileMusicIcon } from 'lucide-svelte';

  const { content_id }: { content_id: string } = $props();

  const content = $derived($contents.get(content_id));
</script>

{#if content !== undefined}
  <div class='flex'>
    <FileMusicIcon />
    <span>{content.title}</span>
  </div>
{/if}
