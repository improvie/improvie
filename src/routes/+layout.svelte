<script lang='ts'>
  import type { Snippet } from 'svelte';

  import * as Sidebar from '$lib/components/ui/sidebar';
  import { Toaster } from '$lib/components/ui/sonner';
  import AppSidebar from '$lib/features/app-sidebar/index.svelte';
  import TrackPlayer from '$lib/features/track-player/index.svelte';
  import { get_themes } from '$lib/plugin/feature/theme.svelte';
  import { initSlots } from '$lib/stores/index.svelte';
  import { init_items } from '$lib/stores/items';
  import { init_play_items } from '$lib/stores/plays';
  import { tracker } from '$lib/stores/tracker.svelte';
  import { loadTranslations } from '$lib/translations/translations';
  import '../app.css';

  const { children }: { children: Snippet } = $props();

  const defaultLanguage = 'ja';
  loadTranslations(defaultLanguage, '/');

  $effect.pre(() => {
    tracker.init();
    init_items();
    init_play_items();
    get_themes();
  });

  const slots = initSlots();
</script>

<Toaster />

<Sidebar.Provider>
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
