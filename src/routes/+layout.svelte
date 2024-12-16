<script>
    import "../app.css";

    import { loadTranslations } from "$lib/translations/translations";

    const defaultLanguage = "ja";
    loadTranslations(defaultLanguage, "/");
</script>

<slot></slot>
