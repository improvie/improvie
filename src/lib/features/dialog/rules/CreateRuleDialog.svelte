<script lang='ts'>
  import type { RuleType } from '$lib/types/rules';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';
  import RuleContentForm from '$lib/features/form/rules/RuleContentForm.svelte';

  let { target = $bindable(), open = $bindable() }: { target: { rules: RuleType[] }; open: boolean } = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title class='text-center'>Create Rule</Dialog.Title>
    </Dialog.Header>
    <Tabs.Root value='rule' class='min-h-80 text-center'>
      <Tabs.List>
        <Tabs.Trigger value='content'>Content</Tabs.Trigger>
        <Tabs.Trigger value='range'>Range</Tabs.Trigger>
        <Tabs.Trigger value='loop'>Loop</Tabs.Trigger>
        <Tabs.Trigger value='random'>Random</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value='content'>
        <RuleContentForm bind:target />
      </Tabs.Content>
      <Tabs.Content value='range'>Change your password here.</Tabs.Content>
      <Tabs.Content value='loop'>Change your password here.</Tabs.Content>
      <Tabs.Content value='random'>Change your password here.</Tabs.Content>
    </Tabs.Root>
  </Dialog.Content>
</Dialog.Root>
