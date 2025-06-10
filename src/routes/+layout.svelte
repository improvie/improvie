<script lang='ts'>
  import type { Snippet } from 'svelte';
  import * as Sidebar from '$lib/components/ui/sidebar';
  import { Toaster } from '$lib/components/ui/sonner';
  import AppSidebar from '$lib/features/app-sidebar/index.svelte';
  import SettingsDialog from '$lib/features/settings-dialog/index.svelte';
  import TrackPlayer from '$lib/features/track-player/index.svelte';
  import { initSlots } from '$lib/stores/index.svelte';
  import { init_items } from '$lib/stores/items';
  import { init_play_items } from '$lib/stores/plays';
  import { init_theme } from '$lib/stores/theme.svelte';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { loadTranslations } from '$lib/translations/translations';
  import Sealed from './sealed.svelte';
  import '../app.css';

  const { children }: { children: Snippet } = $props();

  const defaultLanguage = 'ja';
  loadTranslations(defaultLanguage, '/');

  $effect.pre(() => {
    init_theme();
    tracker.init();
    init_items();
    init_play_items();
  });

  const slots = initSlots();
</script>

<Toaster />

<Sidebar.Provider class='items-center' open={true} style='--sidebar-width: 10rem; --sidebar-width-mobile: 10rem;'>
  <Sealed />
  <AppSidebar />
  <Sidebar.Inset class='select-none'>
    <header class='flex h-14 shrink-0 items-center gap-2'>
      <div class='flex flex-1 items-center gap-2 px-3'>
        <Sidebar.Trigger class='h-10 w-10 z-50' />
      </div>
      {@render slots.header?.()}
    </header>

    {@render children?.()}
    <TrackPlayer />
  </Sidebar.Inset>
</Sidebar.Provider>

<SettingsDialog />
