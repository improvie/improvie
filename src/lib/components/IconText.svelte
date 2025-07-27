<script lang='ts'>
  import type { Icon as IconType } from '@lucide/svelte';
  import { cn } from '$lib/utils';

  const {
    class: className,
    iconClass,
    icon,
    text,
  }: {
    class?: string;
    iconClass?: string;
    icon: typeof IconType;
    text: string;
  } = $props();

  const Icon = icon;
</script>

<p class={className}>
  <Icon class={cn('mr-1 inline-block align-middle', iconClass)} />
  {text}
</p>
