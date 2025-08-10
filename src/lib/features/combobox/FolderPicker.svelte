<script lang='ts'>
  import * as Command from '$lib/components/ui/command/index.js';
  import { folder_nodes } from '$lib/stores/items';
  import { folders } from '$lib/stores/items/folder';
  import { cn, ULID_NIL } from '$lib/utils.js';
  import { Check } from '@lucide/svelte';

  let { folder_id = $bindable(), open = $bindable() }: { folder_id: string | undefined; open: boolean } = $props();

  type PickItem = {
    id: string;
    hierarchy_name: string;
  };

  function get_folders(id: string): PickItem[] {
    const node = folder_nodes.get(id);
    if (!node) {
      return [];
    }
    const folder = folders.get(id);
    if (!folder) {
      return [];
    }
    let items: PickItem[] = [{
      id,
      hierarchy_name: folder.title,
    }];
    for (const item of node.items) {
      if (item.kind === 'Folder') {
        const children = get_folders(item.id);
        const new_children = children.map(child => ({
          id: child.id,
          hierarchy_name: `${folder.title}/${child.hierarchy_name}`,
        }));
        items = items.concat(new_children);
      }
    }
    return items;
  }

  const items = get_folders(ULID_NIL);

  function closeAndFocusTrigger() {
    open = false;
  }
</script>

<Command.Dialog bind:open>
  <Command.Input placeholder='Search folders...' />
  <Command.List>
    <Command.Empty>No folder found.</Command.Empty>
    <Command.Group>
      {#each items as item}
        <Command.Item
          value={item.id}
          onSelect={() => {
            folder_id = item.id;
            closeAndFocusTrigger();
          }}
        >
          <Check
            class={cn(
              'mr-1 size-4',
              folder_id !== item.id && 'text-transparent',
            )}
          />
          {item.hierarchy_name}
        </Command.Item>
      {/each}
    </Command.Group>
  </Command.List>
</Command.Dialog>
