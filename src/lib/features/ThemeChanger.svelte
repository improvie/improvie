<script lang="ts">
  import LightSwitch from "$lib/components/LightSwitch.svelte";
  import * as Dialog from "$lib/components/ui/dialog";
  import Separator from "$lib/components/ui/separator/separator.svelte";

  let { open }: { open: boolean } = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Title class="sr-only">Theme Changer</Dialog.Title>
    <div class="flex gap-8 items-center justify-center">
      <h2 class="text-lg">Mode</h2>
      <LightSwitch />
    </div>
    <Separator />
  </Dialog.Content>
</Dialog.Root>
