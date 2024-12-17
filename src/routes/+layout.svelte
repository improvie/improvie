<script lang="ts">
    import "../app.css";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import { loadTranslations } from "$lib/translations/translations";
    import Header from "$lib/components/Header.svelte";

    const defaultLanguage = "ja";
    loadTranslations(defaultLanguage, "/");
</script>

<div
    class="grid h-screen grid-rows-[auto_1fr_auto] text-primary-400-500-token hide-scrollbar"
>
    <header>
        <Header />
    </header>
    <div class="grid grid-cols-1 md:grid-cols-[auto_1fr]">
        <aside>
            <Sidebar />
        </aside>
        <slot />
    </div>
</div>

<style></style>
