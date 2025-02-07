import i18n from "sveltekit-i18n";

/** @typedef {import('sveltekit-i18n').Config} */
const config = {
  loaders: [
    {
      locale: "en",
      key: "common",
      loader: async () => (await import("$lib/translations/en/common.json")).default,
    },
    {
      locale: "ja",
      key: "common",
      loader: async () => (await import("$lib/translations/ja/common.json")).default,
    },
  ],
};

export const { t, locale, locales, loading, loadTranslations } = new i18n(
  config,
);
