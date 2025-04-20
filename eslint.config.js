import antfu from '@antfu/eslint-config';

export default antfu({
  ignores: [
    'src-tauri',
    'package-json.json',
    'bun.lock',
    'src/lib/components/ui',
    'src/lib/hooks',
    'src/static/',
    'src/bindings/',
  ],
  stylistic: {
    indent: 2,
    semi: true,
    quotes: 'single',
  },
  rules: {
    'import/no-self-import': 'off',
    '@typescript-eslint/no-namespace': 'off',
  },
  svelte: true,
});
