<script module>
  export { default as ItemPageButtons } from "./Buttons.svelte";
</script>

<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { ImportIcon } from "lucide-svelte";
</script>

<div class="flex">
  <Button variant="outline" class="w-40">
    <ImportIcon class="mr-2 size-4" />Import
  </Button>
</div>
