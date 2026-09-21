import * as React from 'react';

/**
 * Theme: light / dark, master-branch mechanism. The active theme lives in
 * the `.dark` class on <html> and in localStorage under 'theme'. index.html
 * applies the stored value before first paint.
 */
export type Theme = 'light' | 'dark';

export type UseThemeReturn = {
	theme: Theme;
	toggleTheme: () => void;
	setTheme: (theme: Theme) => void;
};

function readTheme(): Theme {
	if (typeof document === 'undefined') return 'light';
	return document.documentElement.classList.contains('dark')
		? 'dark'
		: 'light';
}

function updateFavicon(theme: Theme) {
	if (typeof document === 'undefined') return;
	const existingIcons = document.querySelectorAll("link[rel*='icon']");
	existingIcons.forEach(el => el.remove());

	const color = theme === 'light' ? '%2309090b' : '%23ffffff';
	const newLink = document.createElement('link');
	newLink.rel = 'icon';
	newLink.type = 'image/svg+xml';
	newLink.href = `data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='${color}' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><path d='M12 2v10'/><path d='M18.36 6.64a9 9 0 1 1-12.73 0'/></svg>`;

	document.head.appendChild(newLink);
}

export function useTheme(): UseThemeReturn {
	const [theme, setThemeState] = React.useState<Theme>(readTheme);

	React.useEffect(() => {
		// Keep local state in sync if the class changes elsewhere (the
		// pre-paint script, devtools, another tab).
		const observer = new MutationObserver(() => {
			setThemeState(readTheme());
		});
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class'],
		});
		updateFavicon(readTheme());
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
		updateFavicon(next);
	}, []);

	const toggleTheme = React.useCallback(() => {
		setTheme(readTheme() === 'dark' ? 'light' : 'dark');
	}, [setTheme]);

	return {theme, toggleTheme, setTheme};
}
