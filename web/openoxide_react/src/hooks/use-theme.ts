import * as React from 'react';

/**
 * Panel theme: light → dim → dark → light (OpenShip's three-surface ladder).
 * The active theme lives on <html data-theme="…"> and in localStorage under
 * 'theme'. index.html applies the stored value before first paint.
 */
export type Theme = 'light' | 'dim' | 'dark';

const THEME_ORDER: Theme[] = ['light', 'dim', 'dark'];

export type UseThemeReturn = {
	theme: Theme;
	toggleTheme: () => void;
	setTheme: (theme: Theme) => void;
};

function readTheme(): Theme {
	if (typeof document === 'undefined') return 'dim';
	const attr = document.documentElement.getAttribute('data-theme');
	return attr === 'light' || attr === 'dark' || attr === 'dim'
		? attr
		: 'dim';
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
		// Keep local state in sync if the attribute changes elsewhere (the
		// pre-paint script, devtools, another tab).
		const observer = new MutationObserver(() => {
			setThemeState(readTheme());
		});
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['data-theme'],
		});
		updateFavicon(readTheme());
		return () => observer.disconnect();
	}, []);

	const setTheme = React.useCallback((next: Theme) => {
		document.documentElement.setAttribute('data-theme', next);
		localStorage.setItem('theme', next);
		setThemeState(next);
		updateFavicon(next);
	}, []);

	const toggleTheme = React.useCallback(() => {
		const current = readTheme();
		const next = THEME_ORDER[(THEME_ORDER.indexOf(current) + 1) % THEME_ORDER.length];
		setTheme(next);
	}, [setTheme]);

	return {theme, toggleTheme, setTheme};
}
