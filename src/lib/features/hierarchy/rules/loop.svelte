<script lang='ts'>
  import type { LoopRule } from '$lib/types/rules';
  import { FolderOpenIcon } from 'lucide-svelte';

  let { rule = $bindable() }: { rule: LoopRule } = $props();

</script>

<button class='font-medium'>
  <FolderOpenIcon />
  <span>{rule.times}</span>
</button>
