import type { Config } from "tailwindcss";

import { skeleton } from "@skeletonlabs/tw-plugin";
import { join } from "node:path";
import defaultTheme from "tailwindcss/defaultTheme";

export default {
  darkMode: "selector",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(
      require.resolve("@skeletonlabs/skeleton"),
      "../**/*.{html,js,svelte,ts}",
    ),
  ],

  theme: {
    extend: {
      fontFamily: {
        main: ['"Mamelon"', ...defaultTheme.fontFamily.sans],
      },
    },
  },

  plugins: [
    skeleton({
      themes: {
        preset: [
          { name: "crimson", enhancements: true },
          { name: "gold-nouveau", enhancements: true },
          { name: "hamlindigo", enhancements: true },
          { name: "modern", enhancements: true },
          { name: "rocket", enhancements: true },
          { name: "sahara", enhancements: true },
          { name: "seafoam", enhancements: true },
          { name: "skeleton", enhancements: true },
          { name: "vintage", enhancements: true },
          { name: "wintry", enhancements: true },
        ],
      },
    }),
  ],
} as Config;
