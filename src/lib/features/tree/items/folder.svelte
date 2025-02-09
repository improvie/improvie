<script>
  import { slide } from "svelte/transition";
  import { TreeContent } from ".";
  import { FolderIcon, FolderOpenIcon } from "lucide-svelte";

  export let expanded = false;
  export let name;
  export let files;

  function toggle() {
    expanded = !expanded;
  }
</script>

<button on:click={toggle} class="font-medium">
  {#if expanded}
    <FolderOpenIcon />
  {:else}
    <FolderIcon />
  {/if}
  {name}
</button>

{#if expanded}
  <ul transition:slide={{ duration: 300 }}>
    {#each files as file}
      <li>
        {#if file.type === "folder"}
          <svelte:self {...file} />
        {:else}
          <TreeContent {...file} />
        {/if}
      </li>
    {/each}
  </ul>
{/if}

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
