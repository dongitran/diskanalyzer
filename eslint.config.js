// Basic ESLint flat config for Svelte + TS (professional baseline)
// Expand with @typescript-eslint, eslint-plugin-svelte, etc. as project grows.
import js from '@eslint/js';

export default [
  js.configs.recommended,
  {
    files: ['**/*.ts', '**/*.svelte'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module'
    },
    rules: {
      'no-unused-vars': 'warn',
      'no-undef': 'off' // Svelte/TS handles
    }
  },
  {
    ignores: ['node_modules/**', 'build/**', 'src-tauri/target/**', 'dist/**']
  }
];