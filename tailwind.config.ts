import type { Config } from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme';

const config: Config = {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        main: ['"LINESeedJP"', ...defaultTheme.fontFamily.sans],
      },
    },
  },
};

export default config;
