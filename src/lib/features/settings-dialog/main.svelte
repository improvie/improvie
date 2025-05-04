<script lang='ts'>
  import { Input } from '$lib/components/ui/input/index.js';
  import { Label } from '$lib/components/ui/label/index.js';
  import { FilmIcon } from '@lucide/svelte';
</script>

<div class='flex justify-between'>
  <div class='flex text-left items-center gap-2'>
    <FilmIcon class='size-10' />
    <Label for='document-dir'>Document Directory</Label>
  </div>
  <Input id='document-dir' />
</div>
