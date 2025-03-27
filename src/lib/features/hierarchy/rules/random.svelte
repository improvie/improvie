<script lang='ts'>
  import type { RandomRule } from '$lib/types/rules';
  import { FolderOpenIcon } from 'lucide-svelte';

  let { rule = $bindable() }: { rule: RandomRule } = $props();

</script>

<button class='font-medium'>
  <FolderOpenIcon />
  <span>{rule.duplicate}</span>
</button>
