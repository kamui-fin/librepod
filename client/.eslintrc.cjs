/* eslint-env node */

module.exports = {
    root: true,
    env: { browser: true, es2020: true },
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:@typescript-eslint/recommended-requiring-type-checking",
        "plugin:react-hooks/recommended",
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        project: true,
        tsconfigRootDir: __dirname,
    },
    plugins: ["unused-imports"],
    rules: {
        "@typescript-eslint/no-non-null-assertion": "off",
        "no-unused-vars": "off", // or "@typescript-eslint/no-unused-vars": "off",
        "unused-imports/no-unused-imports": "warn",
        "@typescript-eslint/no-unused-vars": "off",
    },
    ignorePatterns: ["package*.json"],
}
