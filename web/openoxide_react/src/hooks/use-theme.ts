import * as React from 'react';

export type Theme = 'light' | 'dark';

export type UseThemeReturn = {
	theme: Theme;
	toggleTheme: () => void;
	setTheme: (theme: Theme) => void;
};

export function useTheme(): UseThemeReturn {
	const [theme, setThemeState] = React.useState<Theme>(() => {
		if (typeof window !== 'undefined') {
			return document.documentElement.classList.contains('dark')
				? 'dark'
				: 'light';
		}
		return 'light';
	});

	React.useEffect(() => {
		const checkTheme = () => {
			const isDark =
				document.documentElement.classList.contains('dark') ||
				document.body.classList.contains('dark');
			setThemeState(isDark ? 'dark' : 'light');
		};
		checkTheme();
		// Observe changes to the class attribute of the html tag
		const observer = new MutationObserver(checkTheme);
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class'],
		});
		return () => observer.disconnect();
	}, []);

	const setTheme = React.useCallback((next: Theme) => {
		if (next === 'dark') {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
		localStorage.setItem('theme', next);
		setThemeState(next);
	}, []);

	const toggleTheme = React.useCallback(() => {
		setTheme(theme === 'dark' ? 'light' : 'dark');
	}, [theme, setTheme]);

	return {theme, toggleTheme, setTheme};
}
