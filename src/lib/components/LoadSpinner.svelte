<script lang='ts'>
  import type { IconProps } from '@lucide/svelte';
  import { cn } from '$lib/utils';
  import { LoaderCircleIcon } from '@lucide/svelte';

  const {
    class: className,
    ...restProps
  }: IconProps = $props();
</script>

<LoaderCircleIcon
  class={cn('animate-spin', className)}
  {...restProps}
/>
