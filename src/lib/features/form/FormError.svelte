<script lang='ts' module>
  import type { FormPath as _FormPath } from 'sveltekit-superforms';

  type T = Record<string, unknown>;
  type U = _FormPath<T>;
</script>

<script lang='ts' generics='T extends Record<string, unknown>, U extends _FormPath<T>'>
  import type { FsSuperForm } from 'formsnap';
  import type { ClassValue } from 'svelte/elements';
  import { cn } from '$lib/utils';

  const {
    form,
    class: className,
  }: {
    form: FsSuperForm<T, U>;
    class?: ClassValue | undefined | null;
  } = $props();

  const allErrors = $derived(form.allErrors);
</script>

{#if $allErrors.length}
  <span class={cn('text-destructive text-sm font-medium', className)}>
    {$allErrors[0].messages}
  </span>
{/if}
