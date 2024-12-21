<script lang="ts">
    import "../app.css";

  import { loadTranslations } from "$lib/translations/translations";
  import Sidebar from "$lib/features/Sidebar.svelte";

    const defaultLanguage = "ja";
    loadTranslations(defaultLanguage, "/");
</script>

<div class="max-h-screen text-surface-700-200-token">
    <div class="grid grid-cols-1 md:grid-cols-[auto_1fr] h-screen">
        <aside>
            <Sidebar />
        </aside>
        <slot />
    </div>
</div>

<style></style>
