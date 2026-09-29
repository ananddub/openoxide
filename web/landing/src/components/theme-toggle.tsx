import {useCallback, useEffect, useState} from 'react';
import {Moon, Sun} from 'lucide-react';

type Theme = 'light' | 'dark';

const read = (): Theme =>
	typeof document !== 'undefined' &&
	document.documentElement.classList.contains('dark')
		? 'dark'
		: 'light';

/** Same switch the dashboard uses: a class on <html> plus a localStorage key. */
export function ThemeToggle() {
	const [theme, setTheme] = useState<Theme>(read);

	useEffect(() => {
		document.documentElement.classList.toggle('dark', theme === 'dark');
		localStorage.setItem('theme', theme);
	}, [theme]);

	const flip = useCallback(
		() => setTheme(t => (t === 'dark' ? 'light' : 'dark')),
		[],
	);

	return (
		<button
			type="button"
			onClick={flip}
			aria-label={`Switch to ${theme === 'dark' ? 'light' : 'dark'} theme`}
			className="grid size-8 place-items-center rounded-md border border-border text-muted-foreground transition-colors hover:bg-muted hover:text-foreground active:scale-[0.97]">
			{theme === 'dark' ? (
				<Sun size={15} strokeWidth={1.8} aria-hidden />
			) : (
				<Moon size={15} strokeWidth={1.8} aria-hidden />
			)}
		</button>
	);
}
