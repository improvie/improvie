<script lang="ts">
  import "../app.css";

  import { loadTranslations } from "$lib/translations/translations";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import AppSidebar from "$lib/features/app-sidebar/index.svelte";
  import SettingsDialog from "$lib/features/SettingsDialog.svelte";
  import { ModeWatcher } from "mode-watcher";
  import { onMount } from "svelte";
  import { init_items } from "$lib/stores/items";

  const defaultLanguage = "ja";
  loadTranslations(defaultLanguage, "/");

  onMount(() => {
    init_items();
  });
</script>

<ModeWatcher />

<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset>
    <header class="flex h-14 shrink-0 items-center gap-2">
      <div class="flex flex-1 items-center gap-2 px-3">
        <Sidebar.Trigger class="h-10 w-10" />
      </div>
      <div class="ml-auto px-3">
        <SettingsDialog />
      </div>
    </header>

    <div class="flex flex-1 flex-col gap-4 px-4 py-10">
      <slot />
    </div>
  </Sidebar.Inset>
</Sidebar.Provider>

<style></style>
