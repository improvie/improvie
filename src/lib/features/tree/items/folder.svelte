<script lang='ts'>
  import type { Folder } from '$lib/types/item/index';
  import { FolderIcon, FolderOpenIcon } from 'lucide-svelte';

  let { expanded, folder }: { expanded: boolean; folder: Folder } = $props();

  function toggle() {
    expanded = !expanded;
  }
</script>

<button onclick={toggle} class='font-medium'>
  {#if expanded}
    <FolderOpenIcon />
  {:else}
    <FolderIcon />
  {/if}
  <span>{folder.title}</span>
</button>

<style>
  button {
    background-size: 1em 1em;
    border: none;
    font-size: 14px;
    display: flex;
    gap: 3px;
    align-items: center;
    outline: none;
    background: transparent no-repeat;
  }
</style>
