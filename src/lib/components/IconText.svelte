<script lang='ts'>
  import type { Icon as IconType } from '@lucide/svelte';

  const {
    class: className,
    icon,
    text,
  }: {
    class?: string;
    icon: typeof IconType;
    text: string;
  } = $props();

  const Icon = icon;
</script>

<p class={className}>
  <Icon class='mr-1 inline-block align-middle' />
  {text}
</p>
