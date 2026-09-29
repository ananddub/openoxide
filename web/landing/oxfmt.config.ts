import {defineConfig} from 'oxfmt';

export default defineConfig({
	printWidth: 75,
	trailingComma: 'all',
	singleQuote: true,
	arrowParens: 'avoid',
	bracketSpacing: false,
	bracketSameLine: true,
	useTabs: true,
	sortTailwindcss: {
		stylesheet: './src/styles.css',
		functions: ['clsx', 'cn'],
		preserveWhitespace: true,
	},
	ignore: ['dist/**', 'node_modules/**'],
});
