<script lang='ts'>
  import { Button } from '$lib/components/ui/button/index.js';
  import * as Command from '$lib/components/ui/command/index.js';
  import * as Popover from '$lib/components/ui/popover/index.js';
  import { cn } from '$lib/utils.js';
  import { Check, ChevronsUpDown } from 'lucide-svelte';
  import { tick } from 'svelte';

  let { value = $bindable() }: { value: string } = $props();

  const frameworks = [
    {
      value: 'sveltekit',
      label: 'SvelteKit',
    },
    {
      value: 'next.js',
      label: 'Next.js',
    },
    {
      value: 'nuxt.js',
      label: 'Nuxt.js',
    },
    {
      value: 'remix',
      label: 'Remix',
    },
    {
      value: 'astro',
      label: 'Astro',
    },
  ];

  let open = $state(false);
  let triggerRef = $state<HTMLButtonElement>(null!);

  const selectedValue = $derived(
    frameworks.find(f => f.value === value)?.label,
  );

  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger bind:ref={triggerRef}>
    {#snippet child({ props })}
      <Button
        variant='outline'
        class='w-[200px] justify-between'
        {...props}
        role='combobox'
        aria-expanded={open}
      >
        {selectedValue || 'Select a framework...'}
        <ChevronsUpDown class='ml-2 size-4 shrink-0 opacity-50' />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class='w-[200px] p-0'>
    <Command.Root>
      <Command.Input placeholder='Search framework...' />
      <Command.List>
        <Command.Empty>No framework found.</Command.Empty>
        <Command.Group>
          {#each frameworks as framework}
            <Command.Item
              value={framework.value}
              onSelect={() => {
                value = framework.value;
                closeAndFocusTrigger();
              }}
            >
              <Check
                class={cn(
                  'mr-2 size-4',
                  value !== framework.value && 'text-transparent',
                )}
              />
              {framework.label}
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
