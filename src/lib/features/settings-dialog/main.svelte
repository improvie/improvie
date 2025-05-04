<script lang='ts'>
  import type { AppSettings } from '$bindings/settings/index';

  let { settings = $bindable() }: { settings: AppSettings } = $props();
</script>
