<script lang='ts'>
  import { Button } from '$lib/components/ui/button';
  import * as Drawer from '$lib/components/ui/drawer/index.js';
  import Content from '$lib/features/hierarchy/items/content.svelte';

  let { content = $bindable() }: { content: Content | undefined } = $props();

  let open: boolean = $state(content !== undefined);

  $effect(() => {
    open = content !== undefined;
  });

</script>

<Drawer.Root bind:open>
  <Drawer.Content>
    <Drawer.Header>
      <Drawer.Title>Are you sure absolutely sure?</Drawer.Title>
      <Drawer.Description>This action cannot be undone.</Drawer.Description>
    </Drawer.Header>
    <Drawer.Footer>
      <Button>Submit</Button>
      <Drawer.Close>Cancel</Drawer.Close>
    </Drawer.Footer>
  </Drawer.Content>
</Drawer.Root>
