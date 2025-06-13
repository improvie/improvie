<script lang='ts'>
  import type { IconProps, Icon as IconType } from '@lucide/svelte';
  import { cn } from '$lib/utils';

  const {
    icon: Icon,
    class: className,
    filled,
    ...restProps
  }: IconProps & {
    icon: typeof IconType;
    filled: boolean;
  } = $props();
</script>

<Icon
  class={cn(filled && 'fill-current', className)}
  {...restProps}
/>
