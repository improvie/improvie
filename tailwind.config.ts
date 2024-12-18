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
        mplus: ['"M PLUS Rounded 1c"', ...defaultTheme.fontFamily.mono],
      },
    },
  },

  plugins: [
    skeleton({
      themes: {
        preset: [
          {
            name: "wintry",
            enhancements: true,
          },
        ],
      },
    }),
  ],
} as Config;
