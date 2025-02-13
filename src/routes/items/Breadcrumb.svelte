<script module>
  export { default as ItemPageBreadcrumb } from './Breadcrumb.svelte';
</script>

<script lang="ts">
  import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
  import { current_folder_ids } from '$lib/stores/items';
  import { folders } from '$lib/stores/items/folder';
</script>

<Breadcrumb.Root class="px-4">
  <Breadcrumb.List>
    {#each $current_folder_ids as folder_id}
      <Breadcrumb.Item>
        {@const folder = $folders.get(folder_id)}
        {#if folder == undefined}
          <Breadcrumb.Ellipsis />
        {:else}
          <!-- class="text-foreground truncate max-w-20" -->
          <button
            class="truncate max-w-20"
            onclick={() => {
              current_folder_ids.update((v) => {
                let new_v = [];
                for (let i = 0; i < v.length; i++) {
                  const id = v[i];
                  new_v.push(id);
                  if (id == folder_id) {
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
