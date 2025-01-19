<script lang="ts" module>
  import AudioWaveform from "lucide-svelte/icons/audio-waveform";
  import Command from "lucide-svelte/icons/command";
  import House from "lucide-svelte/icons/house";
  import Search from "lucide-svelte/icons/search";
  // This is sample data.
  const data = {
    teams: [
      {
        name: "Acme Inc",
        logo: Command,
        plan: "Enterprise",
      },
      {
        name: "Acme Corp.",
        logo: AudioWaveform,
        plan: "Startup",
      },
      {
        name: "Evil Corp.",
        logo: Command,
        plan: "Free",
      },
    ],
    navMain: [
      {
        title: "Search",
        url: "#",
        icon: Search,
      },
      {
        title: "Home",
        url: "#",
        icon: House,
        isActive: true,
      },
    ],
  };
</script>

<script lang="ts">
  import NavFavorites from "$lib/components/nav-favorites.svelte";
  import NavMain from "$lib/components/nav-main.svelte";
  import NavSecondary from "$lib/components/nav-secondary.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
</script>

<Sidebar.Root class="border-r-0">
  <Sidebar.Header>
    <NavMain items={data.navMain} />
  </Sidebar.Header>
  <Sidebar.Content>
    <!-- TODO: favorites -->
    <NavFavorites favorites={[]} />
  </Sidebar.Content>
  <Sidebar.Footer>
    <NavSecondary />
  </Sidebar.Footer>
</Sidebar.Root>
