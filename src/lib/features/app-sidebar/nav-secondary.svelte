<script lang="ts">
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import ThemeChanger from '$lib/features/ThemeChanger.svelte';
  import { MessageCircleQuestion, Palette, Settings2 } from 'lucide-svelte';

  let open: boolean = $state(false);
</script>

<Sidebar.Group>
  <Sidebar.GroupContent>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <Settings2 />
          <span>Settings</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton
          onclick={() => {
            // BUG: Not set false to open on closed
            // first set false then set true
            open = false;
            open = true;
          }}
        >
          <Palette />
          <span>Theme</span>
        </Sidebar.MenuButton>
        <ThemeChanger {open} />
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
