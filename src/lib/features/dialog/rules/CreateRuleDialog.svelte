<script lang='ts'>
  import type { RuleType } from '$lib/types/rules';
  import * as Dialog from '$lib/components/ui/dialog/index.js';
  import * as Tabs from '$lib/components/ui/tabs/index.js';

  let { target = $bindable(undefined) }: { target: RuleType[] | undefined } = $props();
</script>

<Dialog.Root open={target !== undefined} onOpenChange={(value) => {
  if (!value) {
    target = undefined;
  }
}}>
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
      <Tabs.Content value='content'>Make changes to your account here.</Tabs.Content>
      <Tabs.Content value='range'>Change your password here.</Tabs.Content>
      <Tabs.Content value='loop'>Change your password here.</Tabs.Content>
      <Tabs.Content value='random'>Change your password here.</Tabs.Content>
    </Tabs.Root>
  </Dialog.Content>
</Dialog.Root>
