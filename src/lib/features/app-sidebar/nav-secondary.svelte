<script lang='ts'>
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import ThemeChanger from '$lib/features/ThemeChanger.svelte';
  import { settingsStore, sidebarToggler } from '$lib/stores/open.svelte';
  import { MessageCircleQuestion, Palette, Settings2 } from '@lucide/svelte';

  let is_open_theme: boolean = $state(false);
</script>

<Sidebar.Group>
  <Sidebar.GroupContent>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton
          onclick={() => {
            if (sidebarToggler.isMobile()) {
              sidebarToggler.toggle();
            }
            settingsStore.toggle();
          }}
        >
          <Settings2 />
          <span>Settings</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton
          onclick={() => {
            is_open_theme = !is_open_theme;
          }}
        >
          <Palette />
          <span>Theme</span>
        </Sidebar.MenuButton>
        <ThemeChanger bind:open={is_open_theme} />
      </Sidebar.MenuItem>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <MessageCircleQuestion />
          <span>Help</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.GroupContent>
</Sidebar.Group>
