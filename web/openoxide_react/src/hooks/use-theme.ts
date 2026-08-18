import * as React from 'react';

export type Theme = 'light' | 'dark';

export type UseThemeReturn = {
	theme: Theme;
	toggleTheme: () => void;
	setTheme: (theme: Theme) => void;
};

function updateFavicon(isDark: boolean) {
	if (typeof document === 'undefined') return;
	const iconLink = document.querySelector<HTMLLinkElement>("link[rel~='icon']");
	if (!iconLink) return;
	const color = isDark ? '%23ffffff' : '%2309090b';
	iconLink.href = `data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='${color}' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><path d='M12 2v10'/><path d='M18.36 6.64a9 9 0 1 1-12.73 0'/></svg>`;
}

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
			updateFavicon(isDark);
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
		updateFavicon(next === 'dark');
	}, []);

	const toggleTheme = React.useCallback(() => {
		setTheme(theme === 'dark' ? 'light' : 'dark');
	}, [theme, setTheme]);

	return {theme, toggleTheme, setTheme};
}
