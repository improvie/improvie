<script lang="ts">
  import { FileMusicIcon } from "lucide-svelte";

  export let name;
</script>

<span><FileMusicIcon /> {name}</span>

<style>
  span {
    display: flex;
    align-items: center;
    gap: 2px;
  }
</style>
