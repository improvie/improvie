<script module>
  export { default as ItemPageButtons } from './Buttons.svelte';
</script>

<script lang="ts">
  import CreateItemDialog from '$lib/features/dialog/items/CreateItemDialog.svelte';
</script>

<div class="flex px-4">
  <CreateItemDialog />
</div>
