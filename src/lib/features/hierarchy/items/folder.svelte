<script lang="ts">
  import { folders } from "$lib/stores/items";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";

  const { folder_id }: { folder_id: string } = $props();

  const folder = $folders.get(folder_id);
</script>

{#if folder != undefined}
  <ContextMenu.Root>
    <ContextMenu.Trigger class="h-full w-full">
      <p>{folder.title}</p>
    </ContextMenu.Trigger>
    <ContextMenu.Content>
      <ContextMenu.Item>Rename</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item class="text-destructive">Remove</ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Root>
{/if}
