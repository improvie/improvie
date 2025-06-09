<script lang='ts'>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { set_current_theme } from '$lib/stores/theme.svelte';

  let { open = $bindable() }: { open: boolean } = $props();

  type Theme = {
    name: string;
    value: string;
  };

  const themes: Theme[] = [
    { name: 'Light', value: 'light' },
    { name: 'Dark', value: 'dark' },
  ];
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Title class='sr-only'>Theme Changer</Dialog.Title>
    <div class='flex gap-8 items-center justify-center'>
      <h2 class='text-lg'>Theme</h2>
      <div class='w-full p-4 flex flex-col gap-2'>
        {#each themes as theme}
          <Button
            variant='outline'
            size='sm'
            class='w-full'
            onclick={() => {
              set_current_theme(theme.value);
            }}
          >
            {theme.name}
          </Button>
        {/each}
      </div>
    </div>
    <Separator />
  </Dialog.Content>
</Dialog.Root>
