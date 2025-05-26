import antfu from '@antfu/eslint-config';

export default antfu({
  ignores: [
    'src-tauri',
    'vcpkg_installed',
    'package-json.json',
    'bun.lock',
    'components.json',
    'src/lib/components/ui',
    'src/lib/hooks',
    'src/static/',
    'src/bindings/',
    '.github',
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
