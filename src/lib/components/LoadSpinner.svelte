<script lang='ts'>
  import { cn } from '$lib/utils';
  import { LoaderCircleIcon } from '@lucide/svelte';

  const {
    class: className,
  }: {
    class?: string;
  } = $props();
</script>

<LoaderCircleIcon class={cn('animate-spin', className)} />
