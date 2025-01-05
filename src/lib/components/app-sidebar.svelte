<script lang="ts" module>
  import AudioWaveform from "lucide-svelte/icons/audio-waveform";
  import Blocks from "lucide-svelte/icons/blocks";
  import Calendar from "lucide-svelte/icons/calendar";
  import Command from "lucide-svelte/icons/command";
  import House from "lucide-svelte/icons/house";
  import Inbox from "lucide-svelte/icons/inbox";
  import MessageCircleQuestion from "lucide-svelte/icons/message-circle-question";
  import Search from "lucide-svelte/icons/search";
  import Settings2 from "lucide-svelte/icons/settings-2";
  import Sparkles from "lucide-svelte/icons/sparkles";
  import Trash2 from "lucide-svelte/icons/trash-2";
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
        title: "Ask AI",
        url: "#",
        icon: Sparkles,
      },
      {
        title: "Home",
        url: "#",
        icon: House,
        isActive: true,
      },
      {
        title: "Inbox",
        url: "#",
        icon: Inbox,
        badge: "10",
      },
    ],
    navSecondary: [
      {
        title: "Calendar",
        url: "#",
        icon: Calendar,
      },
      {
        title: "Settings",
        url: "#",
        icon: Settings2,
      },
      {
        title: "Templates",
        url: "#",
        icon: Blocks,
      },
      {
        title: "Trash",
        url: "#",
        icon: Trash2,
      },
      {
        title: "Help",
        url: "#",
        icon: MessageCircleQuestion,
      },
    ],
    favorites: [
      {
        name: "Project Management & Task Tracking",
        url: "#",
        emoji: "📊",
      },
      {
        name: "Family Recipe Collection & Meal Planning",
        url: "#",
        emoji: "🍳",
      },
      {
        name: "Fitness Tracker & Workout Routines",
        url: "#",
        emoji: "💪",
      },
      {
        name: "Book Notes & Reading List",
        url: "#",
        emoji: "📚",
      },
      {
        name: "Sustainable Gardening Tips & Plant Care",
        url: "#",
        emoji: "🌱",
      },
      {
        name: "Language Learning Progress & Resources",
        url: "#",
        emoji: "🗣️",
      },
      {
        name: "Home Renovation Ideas & Budget Tracker",
        url: "#",
        emoji: "🏠",
      },
      {
        name: "Personal Finance & Investment Portfolio",
        url: "#",
        emoji: "💰",
      },
      {
        name: "Movie & TV Show Watchlist with Reviews",
        url: "#",
        emoji: "🎬",
      },
      {
        name: "Daily Habit Tracker & Goal Setting",
        url: "#",
        emoji: "✅",
      },
    ],
    workspaces: [
      {
        name: "Personal Life Management",
        emoji: "🏠",
        pages: [
          {
            name: "Daily Journal & Reflection",
            url: "#",
            emoji: "📔",
          },
          {
            name: "Health & Wellness Tracker",
            url: "#",
            emoji: "🍏",
          },
          {
            name: "Personal Growth & Learning Goals",
            url: "#",
            emoji: "🌟",
          },
        ],
      },
      {
        name: "Professional Development",
        emoji: "💼",
        pages: [
          {
            name: "Career Objectives & Milestones",
            url: "#",
            emoji: "🎯",
          },
          {
            name: "Skill Acquisition & Training Log",
            url: "#",
            emoji: "🧠",
          },
          {
            name: "Networking Contacts & Events",
            url: "#",
            emoji: "🤝",
          },
        ],
      },
      {
        name: "Creative Projects",
        emoji: "🎨",
        pages: [
          {
            name: "Writing Ideas & Story Outlines",
            url: "#",
            emoji: "✍️",
          },
          {
            name: "Art & Design Portfolio",
            url: "#",
            emoji: "🖼️",
          },
          {
            name: "Music Composition & Practice Log",
            url: "#",
            emoji: "🎵",
          },
        ],
      },
      {
        name: "Home Management",
        emoji: "🏡",
        pages: [
          {
            name: "Household Budget & Expense Tracking",
            url: "#",
            emoji: "💰",
          },
          {
            name: "Home Maintenance Schedule & Tasks",
            url: "#",
            emoji: "🔧",
          },
          {
            name: "Family Calendar & Event Planning",
            url: "#",
            emoji: "📅",
          },
        ],
      },
      {
        name: "Travel & Adventure",
        emoji: "🧳",
        pages: [
          {
            name: "Trip Planning & Itineraries",
            url: "#",
            emoji: "🗺️",
          },
          {
            name: "Travel Bucket List & Inspiration",
            url: "#",
            emoji: "🌎",
          },
          {
            name: "Travel Journal & Photo Gallery",
            url: "#",
            emoji: "📸",
          },
        ],
      },
    ],
  };
</script>

<script lang="ts">
  import NavFavorites from "$lib/components/nav-favorites.svelte";
  import NavMain from "$lib/components/nav-main.svelte";
  import NavSecondary from "$lib/components/nav-secondary.svelte";
  import NavWorkspaces from "$lib/components/nav-workspaces.svelte";
  import TeamSwitcher from "$lib/components/team-switcher.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import type { ComponentProps } from "svelte";

  let {
    ref = $bindable(null),
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref class="border-r-0" {...restProps}>
  <Sidebar.Header>
    <TeamSwitcher teams={data.teams} />
    <NavMain items={data.navMain} />
  </Sidebar.Header>
  <Sidebar.Content>
    <NavFavorites favorites={data.favorites} />
    <NavWorkspaces workspaces={data.workspaces} />
    <NavSecondary items={data.navSecondary} class="mt-auto" />
  </Sidebar.Content>
</Sidebar.Root>
