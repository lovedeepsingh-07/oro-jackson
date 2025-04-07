import { defineConfig } from "eslint/config";
import globals from "globals";
import prettier from "eslint-config-prettier";
import js from "@eslint/js";

export default defineConfig([
	prettier,
	{ files: ["**/*.{js,mjs,cjs}"] },
	{ files: ["**/*.{js,mjs,cjs}"], languageOptions: { globals: globals.node } },
	{ files: ["**/*.{js,mjs,cjs}"], plugins: { js }, extends: ["js/recommended"] }
]);
