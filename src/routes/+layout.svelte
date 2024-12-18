<script lang="ts">
    import "../app.css";

    import "@fontsource/m-plus-rounded-1c/100.css";
    import "@fontsource/m-plus-rounded-1c/400.css";
    import "@fontsource/m-plus-rounded-1c/500.css";
    import "@fontsource/m-plus-rounded-1c/900.css";

    import { loadTranslations } from "$lib/translations/translations";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import Header from "$lib/components/Header.svelte";

    const defaultLanguage = "ja";
    loadTranslations(defaultLanguage, "/");
</script>

<div
    class="grid h-screen grid-rows-[auto_1fr_auto] text-primary-400-500-token hide-scrollbar font-mplus"
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
