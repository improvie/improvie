<script lang="ts">
  import { CircleX, Palette } from "lucide-svelte";
  import LightSwitch from "$lib/components/LightSwitch.svelte";
  import { themes } from "$lib/themes";
  import { createDialog, melt } from "@melt-ui/svelte";
  import { debug } from "@tauri-apps/plugin-log";
  import { fade, fly } from "svelte/transition";

  const {
    elements: { trigger, portalled, overlay, content, close },
    states: { open },
  } = createDialog();

  let currentTheme: string = $state(localStorage.getItem("theme") || "wintry");

  $effect(() => {
    debug(`currentTheme = ${currentTheme}`);
    document.body.setAttribute("data-theme", currentTheme);
    localStorage.setItem("theme", currentTheme);
  });
</script>

<button use:melt={$trigger} class="w-20 h-20">
  <Palette />
</button>

{#if $open}
  <div use:melt={$portalled}>
    <div
      use:melt={$overlay}
      class="fixed inset-0 z-50"
      transition:fade={{ duration: 150 }}
    ></div>
    <div
      use:melt={$content}
      class="on_center rounded-xl bg-surface-100-800-token p-6 shadow-lg"
      transition:fly={{
        duration: 150,
        y: 8,
      }}
    >
      <button
        use:melt={$close}
        aria-label="Close"
        class="absolute right-2 top-2 inline-flex h-6 w-6 btn-icon bg-initial rounded-full"
      >
        <CircleX />
      </button>
      <!-- body -->
      <div class="flex flex-col pt-2 gap-4 items-center">
        <div class="flex gap-8">
          <h2 class="text-lg">モード</h2>
          <LightSwitch />
        </div>
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
      </div>
    </div>
  </div>
{/if}
