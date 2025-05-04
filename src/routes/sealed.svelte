<script lang='ts'>
  import { useSidebar } from '$lib/components/ui/sidebar';
  import { sidebarToggler } from '$lib/stores/open.svelte';

  const sidebar = useSidebar();
  sidebarToggler.toggle = () => sidebar.toggle();
</script>
