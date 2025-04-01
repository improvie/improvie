<script lang='ts'>
  import type { Snippet } from 'svelte';

  import * as Sidebar from '$lib/components/ui/sidebar';
  import AppSidebar from '$lib/features/app-sidebar/index.svelte';
  import TrackPlayer from '$lib/features/track-player/index.svelte';
  import { initSlots } from '$lib/stores/index.svelte';
  // import SettingsDialog from '$lib/features/SettingsDialog.svelte';
  import { init_items } from '$lib/stores/items';
  import { init_play_items } from '$lib/stores/plays';
  import { loadTranslations } from '$lib/translations/translations';
  import { ModeWatcher } from 'mode-watcher';
  import { onMount } from 'svelte';
  import '../app.css';

  const { children }: { children: Snippet } = $props();

  const defaultLanguage = 'ja';
  loadTranslations(defaultLanguage, '/');

  onMount(() => {
    init_items();
    init_play_items();
  });

  const slots = initSlots();
</script>

<ModeWatcher />

<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset>
    <header class='flex h-14 shrink-0 items-center gap-2'>
      <div class='flex flex-1 items-center gap-2 px-3'>
        <Sidebar.Trigger class='h-10 w-10' />
      </div>
      {@render slots.header?.()}
    </header>

    {@render children?.()}
    <TrackPlayer />
  </Sidebar.Inset>
</Sidebar.Provider>

<style></style>
