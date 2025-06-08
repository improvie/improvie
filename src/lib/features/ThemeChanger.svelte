<script lang='ts'>
  import type { ThemeFeature } from '$lib/stores/theme.svelte';
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Dialog from '$lib/components/ui/dialog';
  import Separator from '$lib/components/ui/separator/separator.svelte';
  import { get_themes, set_current_theme } from '$lib/stores/theme.svelte';
  import { onMount } from 'svelte';

  let { open = $bindable() }: { open: boolean } = $props();

  const themes: ThemeFeature[] = $state([]);

  onMount(() => {
    get_themes().then((res_themes) => {
      res_themes.forEach((theme) => {
        themes.push(theme);
      });
    });
  });
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
              set_current_theme(theme);
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
