<script module>
  export { default as PlayPageBreadcrumb } from './Breadcrumb.svelte';
</script>

<script lang='ts'>
  import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
  import { current_play_folder_ids } from '$lib/stores/plays';
  import { play_folders } from '$lib/stores/plays/folder';
</script>

<Breadcrumb.Root class='px-4'>
  <Breadcrumb.List>
    {#each $current_play_folder_ids as folder_id}
      <Breadcrumb.Item>
        {@const folder = $play_folders.get(folder_id)}
        {#if folder === undefined}
          <Breadcrumb.Ellipsis />
        {:else}
          <button
            class='truncate max-w-60'
            onclick={() => {
              current_play_folder_ids.update((v) => {
                const new_v = [];
                for (let i = 0; i < v.length; i++) {
                  const id = v[i];
                  new_v.push(id);
                  if (id === folder_id) {
                    return new_v;
                  }
                }
                return new_v;
              });
            }}>{folder.title}</button
          >
        {/if}
      </Breadcrumb.Item>
      <Breadcrumb.Separator />
    {/each}
  </Breadcrumb.List>
</Breadcrumb.Root>
