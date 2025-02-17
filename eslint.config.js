import antfu from '@antfu/eslint-config';

export default antfu({
  ignores: ['src-tauri', 'src/lib/components/ui', 'src/lib/hooks'],
  stylistic: {
    indent: 2,
    semi: true,
    quotes: 'single',
  },
  rules: {
    'import/no-self-import': 'off',
  },
  svelte: true,
});
