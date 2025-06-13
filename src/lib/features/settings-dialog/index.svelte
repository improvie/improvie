<script lang='ts'>
  import type { AppSettings } from '$bindings/settings';
  import { action_get_app_settings, action_set_app_settings } from '$lib/action/settings';
  import LoadSpinner from '$lib/components/LoadSpinner.svelte';
  import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import { settingsStore } from '$lib/stores/open.svelte';
  import Link from '@lucide/svelte/icons/link';
  import Settings from '@lucide/svelte/icons/settings';
  import { onMount } from 'svelte';
  import AdvancedElement from './advanced.svelte';
  import MainElement from './main.svelte';
  import PluginElement from './plugin.svelte';

  let selected = $state<string | null>(null);

  const data = {
    nav: [
      { name: 'Plugin', icon: Link, element: PluginElement },
      { name: 'Advanced', icon: Settings, element: AdvancedElement },
    ],
  };

  let settings: AppSettings | undefined = $state();

  async function init() {
    settings = await action_get_app_settings();
  }

  $effect(() => {
    if (settings !== undefined) {
      action_set_app_settings(settings);
    }
  });

  onMount(() => {
    init();
  });
</script>

<Dialog.Root bind:open={settingsStore.state}>
  <Dialog.Content class='h-dvh overflow-hidden select-none p-0 max-h-[98%] md:max-h-[90%] md:max-w-[700px] lg:max-w-[800px] xl:max-w-[900px] 2xl:max-w-[1000px]'>
    <Sidebar.Provider class='items-start' open={true} style='--sidebar-width: 10rem;'>
      <Sidebar.Root collapsible='icon'>
        <Sidebar.Header>
          <Sidebar.MenuButton size='lg' isActive={selected === null} onclick={() => (selected = null)}>
            <div
              class='flex aspect-square size-8 items-center justify-center rounded-lg'
            >
              <img src='/logo.png' alt='Logo' class='h-8 w-8 rounded-full' />
            </div>
            <div class='flex flex-col gap-1 leading-none'>
              <span>{PKG.name}</span>
              <span>v{PKG.version}</span>
            </div>
          </Sidebar.MenuButton>
        </Sidebar.Header>
        <Sidebar.Separator />
        <Sidebar.Content>
          <Sidebar.Group>
            <Sidebar.GroupContent>
              <Sidebar.Menu>
                {#each data.nav as item (item.name)}
                  <Sidebar.MenuItem>
                    <Sidebar.MenuButton isActive={item.name === selected}>
                      {#snippet child({ props })}
                        <a {...props} onclick={() => (selected = item.name)}>
                          <item.icon />
                          <span>{item.name}</span>
                        </a>
                      {/snippet}
                    </Sidebar.MenuButton>
                  </Sidebar.MenuItem>
                {/each}
              </Sidebar.Menu>
            </Sidebar.GroupContent>
          </Sidebar.Group>
        </Sidebar.Content>
      </Sidebar.Root>
      <main class='flex max-h-[calc(98%-20px)] md:max-h-[calc(88%-20px)] flex-1 flex-col overflow-hidden'>
        <header
          class='flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12'
        >
          <div class='flex items-center gap-2 px-4'>
            <Sidebar.Trigger class='h-10 w-10' />
            <Breadcrumb.Root>
              <Breadcrumb.List>
                <Breadcrumb.Item>
                  <Breadcrumb.Link onclick={() => selected = null}>Settings</Breadcrumb.Link>
                </Breadcrumb.Item>
                {#if selected !== null}
                  <Breadcrumb.Separator class='hidden md:block' />
                  <Breadcrumb.Item>
                    <Breadcrumb.Page>{selected}</Breadcrumb.Page>
                  </Breadcrumb.Item>
                {/if}
              </Breadcrumb.List>
            </Breadcrumb.Root>
          </div>
        </header>
        <div class='flex flex-1 flex-col gap-4 overflow-y-auto p-4'>
          {#if settings === undefined}
            <LoadSpinner />
          {:else}
            {#if selected === null}
              <MainElement bind:settings />
            {:else}
              {@const Element = data.nav.find(item => item.name === selected)!.element}
              <Element />
            {/if}
          {/if}
        </div>
      </main>
    </Sidebar.Provider>
  </Dialog.Content>
</Dialog.Root>
