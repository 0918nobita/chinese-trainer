/** @type {import('eslint').Linter.Config} */
module.exports = {
    env: {
        browser: true,
        es2021: true,
        node: true,
    },
    extends: ['eslint:recommended', 'plugin:prettier/recommended'],
    parserOptions: {
        sourceType: 'module',
        ecmaVersion: 'latest',
    },
    overrides: [
        {
            files: ['**/*.ts'],
            extends: [
                'plugin:@typescript-eslint/recommended',
                'plugin:@typescript-eslint/recommended-requiring-type-checking',
            ],
            parser: '@typescript-eslint/parser',
            parserOptions: {
                tsconfigRootDir: __dirname,
                project: './tsconfig.json',
            },
            plugins: ['@typescript-eslint'],
        },
        {
            files: ['**/*.tsx'],
            extends: [
                'plugin:@typescript-eslint/recommended',
                'plugin:@typescript-eslint/recommended-requiring-type-checking',
                'plugin:jsx-a11y/recommended',
                'plugin:react/recommended',
                'plugin:react/jsx-runtime',
                'plugin:react-hooks/recommended',
            ],
            parser: '@typescript-eslint/parser',
            parserOptions: {
                tsconfigRootDir: __dirname,
                project: './tsconfig.json',
            },
            plugins: ['@typescript-eslint', 'jsx-a11y', 'react', 'react-hooks'],
            settings: {
                react: {
                    version: 'detect',
                },
            },
        },
    ],
};
