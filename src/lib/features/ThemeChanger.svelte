<script lang="ts">
  import LightSwitch from "$lib/components/LightSwitch.svelte";
  import { themes } from "$lib/themes";
  import { debug } from "@tauri-apps/plugin-log";
  import * as Dialog from "$lib/components/ui/dialog";
  import Separator from "$lib/components/ui/separator/separator.svelte";

  let currentTheme: string = $state(localStorage.getItem("theme") || "wintry");

  $effect(() => {
    debug(`currentTheme = ${currentTheme}`);
    document.body.setAttribute("data-theme", currentTheme);
    localStorage.setItem("theme", currentTheme);
  });

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
    <nav class="list-nav max-h-64 lg:max-h-[500px] overflow-y-auto">
      <ul>
        {#each themes as theme}
          <li>
            <button
              class="option w-full h-full"
              class:bg-primary-active-token={theme.file == currentTheme}
              onclick={() => (currentTheme = theme.file)}
            >
              <span>{theme.icon}</span>
              <span class="flex-auto text-left">{theme.name}</span>
            </button>
          </li>
        {/each}
      </ul>
    </nav>
  </Dialog.Content>
</Dialog.Root>
