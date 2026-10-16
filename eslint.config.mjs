import tsParser from '@typescript-eslint/parser';

export default [{ ignores: ['docs/website/assets/path-selection.d.ts'] }, {
  files: ['docs/website/assets/**/*.{js,mjs,ts,mts}', 'scripts/build_frontend.mjs'],
  rules: {
    'constructor-super': 'error',
    'for-direction': 'error',
    'no-constant-condition': 'error',
    'no-dupe-args': 'error',
    'no-dupe-class-members': 'error',
    'no-dupe-else-if': 'error',
    'no-duplicate-case': 'error',
    'no-duplicate-imports': 'error',
    'no-unreachable': 'error',
    'no-unsafe-finally': 'error',
    'use-isnan': 'error',
    'valid-typeof': 'error',
  },
}, {
  files: ['docs/website/assets/**/*.{ts,mts}'],
  languageOptions: { parser: tsParser },
}];
