<script lang='ts'>
  import type { ItemNode } from '$lib/types/item';
  import { folder_nodes } from '$lib/stores/items';
  import { folders } from '$lib/stores/items/folder';
  import { slide } from 'svelte/transition';
  import { TreeContent, TreeFolder } from '.';
  import Self from './node.svelte';

  const { expanded = true, folder_id }: { expanded?: boolean; folder_id: string } = $props();

  const folder = $derived($folders.get(folder_id));
  const children: ItemNode[] = $derived(
    $folder_nodes.get(folder_id)?.items.sort((a, b) => a.sort_order - b.sort_order) || [],
  );
</script>

{#if folder !== undefined}
  <TreeFolder {expanded} {folder} />

  {#if expanded}
    <ul transition:slide={{ duration: 300 }}>
      {#each children as child}
        <li>
          {#if child.kind === 'Folder'}
            <Self folder_id={child.id} />
          {:else if child.kind === 'Content'}
            <TreeContent content_id={child.id} /> -->
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
{/if}

<style>
  ul {
    padding: 0.2em 0 0 0.5em;
    margin: 0 0 0 0.5em;
    list-style: none;
    border-left: 2px solid #555353;
  }

  li {
    padding: 0.2em 1px;
  }
</style>
