<script lang='ts'>
  import type { RuleType } from '$lib/types/rules';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import RuleContentForm from '$lib/features/form/rules/RuleContentForm.svelte';
  import RuleLoopForm from '$lib/features/form/rules/RuleLoopForm.svelte';
  import RuleRandomForm from '$lib/features/form/rules/RuleRandomForm.svelte';
  // import RuleRangeForm from '$lib/features/form/rules/RuleRangeForm.svelte';

  let { add_rule, open = $bindable() }: { add_rule: (new_rule: RuleType) => void; open: boolean } = $props();

</script>

<Dialog.Root bind:open>
  <Dialog.Content class='sm:max-w-xl md:max-w-3xl'>
    <Dialog.Header>
      <Dialog.Title class='text-center'>Create Rule</Dialog.Title>
    </Dialog.Header>
    <Tabs.Root value='content' class='min-h-80 text-center'>
      <Tabs.List>
        <Tabs.Trigger value='content'>Content</Tabs.Trigger>
        <!-- <Tabs.Trigger value='range'>Range</Tabs.Trigger> -->
        <Tabs.Trigger value='loop'>Loop</Tabs.Trigger>
        <Tabs.Trigger value='random'>Random</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value='content'>
        <RuleContentForm bind:add_rule />
      </Tabs.Content>
      <!-- <Tabs.Content value='range'> -->
      <!--   <RuleRangeForm bind:add_rule /> -->
      <!-- </Tabs.Content> -->
      <Tabs.Content value='loop'>
        <RuleLoopForm bind:add_rule />
      </Tabs.Content>
      <Tabs.Content value='random'>
        <RuleRandomForm bind:add_rule />
      </Tabs.Content>
    </Tabs.Root>
  </Dialog.Content>
</Dialog.Root>
