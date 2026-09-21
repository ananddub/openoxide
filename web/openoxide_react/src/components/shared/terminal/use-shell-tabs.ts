import {useCallback, useRef, useState} from 'react';
import type {ShellEntry, ShellStatus} from './types';

/** Manages the capped (≤3) set of shell tabs: add / close / per-tab status,
 * keeping the active tab valid as tabs come and go. */
export function useShellTabs() {
	const counterRef = useRef(1);
	const [shells, setShells] = useState<ShellEntry[]>(() => [
		{id: 'shell-1', label: 'Shell 1', status: 'disconnected'},
	]);
	const [activeId, setActiveId] = useState('shell-1');

	const setStatus = useCallback((id: string, status: ShellStatus) => {
		setShells(prev => {
			const current = prev.find(s => s.id === id);
			if (!current || current.status === status) return prev;
			return prev.map(s => (s.id === id ? {...s, status} : s));
		});
	}, []);

	const addShell = useCallback(() => {
		setShells(prev => {
			if (prev.length >= 3) return prev;
			counterRef.current += 1;
			const entry = {
				id: `shell-${counterRef.current}-${Date.now()}`,
				label: `Shell ${counterRef.current}`,
				status: 'disconnected' as ShellStatus,
			};
			setActiveId(entry.id);
			return [...prev, entry];
		});
	}, []);

	const closeShell = useCallback((id: string) => {
		setShells(prev => {
			const idx = prev.findIndex(s => s.id === id);
			if (idx < 0) return prev;
			const remaining = prev.filter(s => s.id !== id);
			setActiveId(current => {
				if (current !== id) return current;
				if (remaining.length === 0) return '';
				return remaining[Math.max(0, idx - 1)].id;
			});
			return remaining;
		});
	}, []);

	return {shells, activeId, setActiveId, setStatus, addShell, closeShell};
}
