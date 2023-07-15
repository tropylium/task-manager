/* eslint-env node */

module.exports = {
  root: true,
  env: { browser: true, es2020: true },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
    'plugin:react-hooks/recommended',
    'plugin:jsdoc/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: true,
    tsconfigRootDir: __dirname,
  },
  plugins: ['react-refresh'],
  rules: {
    'react-refresh/only-export-components': [
      'warn',
      { allowConstantExport: true },
    ],
    '@typescript-eslint/no-non-null-assertion': 'off',
    'no-tabs': 'error',
    '@typescript-eslint/naming-convention': [
      'error',
      // allow UPPER_CASE enum members:
      {
        selector: 'enumMember',
        format: ['UPPER_CASE', 'camelCase', 'PascalCase']
      },
      // ... and the rest is just restating the naming-convention defaults (which is necessary,
      // because overriding this option erases the default naming convention rules and replaces them with these rules)
      {
        selector: 'default',
        format: ['camelCase'],
        leadingUnderscore: 'allow',
        trailingUnderscore: 'allow',
      },

      {
        selector: 'variable',
        format: ['camelCase', 'UPPER_CASE'],
        leadingUnderscore: 'allow',
        trailingUnderscore: 'allow',
      },

      {
        selector: 'function',
        format: ['camelCase', 'PascalCase'],
        leadingUnderscore: 'allow',
        trailingUnderscore: 'allow',
      },

      {
        selector: 'typeLike',
        format: ['PascalCase'],
      },
    ],

    '@typescript-eslint/explicit-member-accessibility': 'error',
    '@typescript-eslint/no-inferrable-types': 'warn',
    '@typescript-eslint/no-for-in-array': 'error',
    '@typescript-eslint/prefer-for-of': 'warn',
    '@typescript-eslint/no-unsafe-argument': 'warn',
    '@typescript-eslint/prefer-nullish-coalescing': 'warn',
    '@typescript-eslint/prefer-readonly': 'error',
    '@typescript-eslint/strict-boolean-expressions': 'error',
    '@typescript-eslint/no-unnecessary-boolean-literal-compare': 'error',
    '@typescript-eslint/switch-exhaustiveness-check': 'error',
    'comma-dangle': ['error', 'always-multiline'],

    'jsdoc/no-types': 'error', // forbid types in @param, because they're redundant with TS declaration
    'jsdoc/require-param-type': 'off', // ditto
    'jsdoc/require-returns-type': 'off', // ditto
    'jsdoc/tag-lines': 'off', // too picky about spacing
  },
}
