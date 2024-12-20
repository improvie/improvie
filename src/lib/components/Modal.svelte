<script lang="ts">
    import { backIn, backOut } from "svelte/easing";

    // imports
    import { fade, scale } from "svelte/transition";

    let {
        children,
        triggetRef = undefined,
        isOpen = false,
        title = "Modal title",
        role = "dialog",
    } = $props();

    // local props
    let buttonRef: HTMLButtonElement | undefined = $state(undefined);

    // functions
    const handleClose = () => (isOpen = false);
    const handleEsc = (e: KeyboardEvent) => e.key === "Escape" && handleClose();

    // lifecycle
    $effect(() => {
        if (isOpen) {
            buttonRef?.focus();
        } else {
            triggetRef?.focus();
        }
    });
</script>

{#if isOpen}
    <aside
        onkeydown={handleEsc}
        aria-labelledby="modal-heading"
        aria-modal="true"
        tabIndex={-1}
        {role}
        in:fade
        out:fade
        onclick={handleClose}
        class="overlay"
    >
        <div
            in:scale={{ start: 0.8, easing: backOut }}
            out:scale={{ start: 0.8, easing: backIn }}
            class="box"
        >
            <header>
                <h3 id="modal-heading">{title}</h3>
                <button
                    aria-label="Close modal"
                    bind:this={buttonRef}
                    onclick={handleClose}>&#10005;</button
                >
            </header>
            <main>
                {#if children}
                    {@render children()}
                {:else}
                    <span>No content provided</span>
                {/if}
            </main>
        </div>
    </aside>
{/if}

<style>
    * {
        box-sizing: border-box;
    }
    aside {
        position: fixed;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 20px;
    }

    aside .box {
        background: #fff;
        max-width: 700px;
        border-radius: 4px;
        position: relative;
        box-sizing: 0 0 20px 0px rgba(0, 0, 0, 0.3);
    }

    aside .box header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        border-bottom: 1px solid #ededed;
    }

    aside .box header h3 {
        margin: 0;
        font-size: 25px;
    }

    aside .box header button {
        background: none;
        border: none;
        padding: 0;
        font-size: 20px;
        position: absolute;
        right: 20px;
        color: #ccc;
        font-weight: lighter;
        cursor: pointer;
    }

    aside .box main {
        padding: 20px;
    }
</style>
