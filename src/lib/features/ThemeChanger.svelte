<script lang="ts">
    import Icon from "@iconify/svelte";
    import { createDialog, melt } from "@melt-ui/svelte";
    import { fade, fly } from "svelte/transition";
    const {
        elements: {
            trigger,
            portalled,
            overlay,
            content,
            title,
            description,
            close,
        },
        states: { open },
    } = createDialog({
        openFocus: "#theme-changer",
    });
</script>

<button use:melt={$trigger} class="h-20 w-20"
    ><Icon icon="mdi:color" width="36" height="36" class="m-auto" /></button
>

{#if $open}
    <div use:melt={$portalled} id="theme-changer">
        <div
            use:melt={$overlay}
            class="fixed inset-0 z-50"
            transition:fade={{ duration: 150 }}
        ></div>
        <div
            use:melt={$content}
            class="on_center rounded-xl bg-surface-50-900-token p-6 shadow-lg"
            transition:fly={{
                duration: 150,
                y: 8,
            }}
        >
            <h2 use:melt={$title}>Dialog Title</h2>
            <p use:melt={$description}>Dialog description</p>
            <!-- ISSUE: disable auto focus on open -->
            <button
                use:melt={$close}
                aria-label="Close"
                class="absolute right-4 top-4 inline-flex h-6 w-6 appearance-none btn-icon variant-ghost-primary rounded-full"
            >
                <Icon icon="mdi:close" width="24" height="24" />
            </button>
        </div>
    </div>
{/if}
