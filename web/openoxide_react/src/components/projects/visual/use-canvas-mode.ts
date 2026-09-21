import {useCallback, useEffect, useState} from 'react';

export type CanvasMode = 'select' | 'arrow';

/** Canvas interaction mode with keyboard shortcuts: A = arrow, Esc/V/S = select. */
export function useCanvasMode(onExitArrow: () => void) {
	const [mode, setMode] = useState<CanvasMode>('select');

	const pickMode = useCallback(
		(m: CanvasMode) => {
			setMode(m);
			onExitArrow();
		},
		[onExitArrow],
	);

	useEffect(() => {
		const h = (e: KeyboardEvent) => {
			const tag = (e.target as HTMLElement).tagName;
			if (tag === 'INPUT' || tag === 'TEXTAREA') return;
			if (e.key === 'a' || e.key === 'A') setMode('arrow');
			if (['Escape', 's', 'S', 'v', 'V'].includes(e.key)) {
				setMode('select');
				onExitArrow();
			}
		};
		window.addEventListener('keydown', h);
		return () => window.removeEventListener('keydown', h);
	}, [onExitArrow]);

	return {mode, pickMode};
}
