import tsParser from '@typescript-eslint/parser';

export default [{ ignores: ['docs/website/src/workers/path-selection.d.ts'] }, {
  files: ['docs/website/src/**/*.{js,mjs,ts,mts}', 'scripts/{build_frontend,generate_website_contracts,validate_website_data,website_contracts}.mjs'],
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
  files: ['docs/website/src/**/*.{ts,mts}'],
  languageOptions: { parser: tsParser },
}];
