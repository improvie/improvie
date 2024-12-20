<script lang="ts">
    import "../app.css";

    import "@fontsource/m-plus-rounded-1c/100.css";
    import "@fontsource/m-plus-rounded-1c/400.css";
    import "@fontsource/m-plus-rounded-1c/500.css";
    import "@fontsource/m-plus-rounded-1c/900.css";

    import { loadTranslations } from "$lib/translations/translations";
    import Sidebar from "$lib/features/Sidebar.svelte";

    const defaultLanguage = "ja";
    loadTranslations(defaultLanguage, "/");
</script>

<div class="max-h-screen text-surface-700-200-token font-mplus">
    <div class="grid grid-cols-1 md:grid-cols-[auto_1fr] h-screen">
        <aside>
            <Sidebar />
        </aside>
        <slot />
    </div>
</div>

<style></style>
