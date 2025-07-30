<script lang='ts'>
  import * as Command from '$lib/components/ui/command/index.js';
  import { folder_nodes } from '$lib/stores/items';
  import { contents } from '$lib/stores/items/content';
  import { folders } from '$lib/stores/items/folder';
  import { cn, ULID_NIL } from '$lib/utils.js';
  import { Check } from '@lucide/svelte';

  let { content_id = $bindable(), open = $bindable() }: { content_id: string | undefined; open: boolean } = $props();

  type PickItem = {
    id: string;
    hierarchy_name: string;
  };

  function get_content(id: string): PickItem[] {
    const node = folder_nodes.get(id);
    if (!node) {
      return [];
    }
    const folder = folders.get(id);
    if (!folder) {
      return [];
    }
    let items: PickItem[] = [];
    for (const item of node.items) {
      if (item.kind === 'Folder') {
        const children = get_content(item.id);
        const new_children = children.map(child => ({
          id: child.id,
          hierarchy_name: `${folder.title}/${child.hierarchy_name}`,
        }));
        items = items.concat(new_children);
      }
      else if (item.kind === 'Content') {
        const content = contents.get(item.id);
        if (!content) {
          continue;
        }
        items.push({
          id: item.id,
          hierarchy_name: `${folder.title}/${content.title}`,
        });
      }
    }
    return items;
  }

  const items = get_content(ULID_NIL);

  function closeAndFocusTrigger() {
    open = false;
  }
</script>

<Command.Dialog bind:open>
  <Command.Input placeholder='Search contents...' />
  <Command.List>
    <Command.Empty>No content found.</Command.Empty>
    <Command.Group>
      {#each items as item}
        <Command.Item
          value={item.id}
          onSelect={() => {
            content_id = item.id;
            closeAndFocusTrigger();
          }}
        >
          <Check
            class={cn(
              'mr-1 size-4',
              content_id !== item.id && 'text-transparent',
            )}
          />
          {item.hierarchy_name}
        </Command.Item>
      {/each}
    </Command.Group>
  </Command.List>
</Command.Dialog>
