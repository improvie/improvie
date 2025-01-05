<script lang="ts">
  import { createDialog, melt } from "@melt-ui/svelte";
  import { CircleX, FilePlus } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";

  const {
    elements: { trigger, portalled, overlay, content, close },
    states: { open },
  } = createDialog();
</script>

<button use:melt={$trigger} class="w-20 h-20">
  <FilePlus />
</button>

{#if $open}
  <div use:melt={$portalled}>
    <div
      use:melt={$overlay}
      class="fixed inset-0 z-50"
      transition:fade={{ duration: 150 }}
    ></div>
    <div
      use:melt={$content}
      class="on_center rounded-xl bg-surface-100-800-token p-6 shadow-lg"
      transition:fly={{
        duration: 150,
        y: 8,
      }}
    >
      <button
        use:melt={$close}
        aria-label="Close"
        class="absolute right-2 top-2 inline-flex h-6 w-6 btn-icon bg-initial rounded-full"
      >
        <CircleX />
      </button>
      <!-- body -->
      <div class="flex flex-col pt-2 gap-4 items-center">
        <span>Hey</span>
      </div>
    </div>
  </div>
{/if}
