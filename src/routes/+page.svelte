<script lang="ts">
    import { t } from "$lib/translations/translations";
    import { invoke } from "@tauri-apps/api/core";

    let name = $state("");
    let greetMsg = $state("");

    async function greet(event: Event) {
        event.preventDefault();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        greetMsg = await invoke("greet", { name });
    }
</script>

<main class="flex flex-col w-full justify-center items-center">
    <h1 class="text-secondary-500">{$t("common.welcome")}</h1>

    <div class="text-primary-500">Sample Text</div>

    <div class="flex">
        <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
        </a>
        <a href="https://kit.svelte.dev" target="_blank">
            <img
                src="/svelte.svg"
                class="logo svelte-kit"
                alt="SvelteKit Logo"
            />
        </a>
    </div>
    <p class="text-secondary-500">
        Click on the Tauri, Vite, and SvelteKit logos to learn more.
    </p>

    <form class="row" onsubmit={greet}>
        <input
            id="greet-input"
            placeholder="Enter a name..."
            bind:value={name}
        />
        <button type="submit">Greet</button>
    </form>
    <p class="text-secondary-500">{greetMsg}</p>
</main>

<style lang="postcss">
    .logo {
        @apply w-20 h-20 p-2;
    }
</style>
