<script lang="ts">
  import { contents } from "$lib/stores/items";
  import { FileMusicIcon } from "lucide-svelte";

  let { content_id }: { content_id: string } = $props();

  let content = $derived($contents.get(content_id));
</script>

{#if content != undefined}
  <span><FileMusicIcon /> {content_id}</span>
{/if}

<style>
  span {
    display: flex;
    align-items: center;
    gap: 2px;
  }
</style>
