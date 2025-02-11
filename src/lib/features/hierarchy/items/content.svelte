<script lang="ts">
  import { contents } from "$lib/stores/items";
  import { FileMusicIcon } from "lucide-svelte";

  const { content_id }: { content_id: string } = $props();

  const content = $derived($contents.get(content_id));
</script>

{#if content != undefined}
  <span><FileMusicIcon /> {content_id}</span>
{/if}
