import {useEffect, useMemo, useRef, useState} from 'react';
import {Terminal as Xterm} from '@xterm/xterm';
import {FitAddon} from '@xterm/addon-fit';
import type {ITheme} from '@xterm/xterm';
import '@xterm/xterm/css/xterm.css';
import {useTerminalSocket} from '#/hooks/terminal/use-terminal-socket';
import {useTheme} from '#/hooks/use-theme';
import type {ShellStatus} from './types';

// xterm resolves literal colors only — CSS vars never reach it, so the
// palette is spelled out per mode and swapped live on theme change.
const DARK_TERMINAL_THEME: ITheme = {
	background: '#0a0a0a',
	foreground: '#e6e6e6',
	cursor: '#22c55e',
	cursorAccent: '#0a0a0a',
	selectionBackground: 'rgba(255,255,255,0.16)',
	black: '#000000',
	red: '#f87171',
	green: '#4ade80',
	yellow: '#facc15',
	blue: '#60a5fa',
	magenta: '#c084fc',
	cyan: '#22d3ee',
	white: '#e5e5e5',
	brightBlack: '#525252',
	brightRed: '#fca5a5',
	brightGreen: '#86efac',
	brightYellow: '#fde047',
	brightBlue: '#93c5fd',
	brightMagenta: '#d8b4fe',
	brightCyan: '#67e8f9',
	brightWhite: '#fafafa',
};

const LIGHT_TERMINAL_THEME: ITheme = {
	background: '#f4f4f5',
	foreground: '#1a1a1a',
	cursor: '#16a34a',
	cursorAccent: '#ffffff',
	selectionBackground: 'rgba(0,0,0,0.12)',
	black: '#000000',
	red: '#dc2626',
	green: '#16a34a',
	yellow: '#ca8a04',
	blue: '#2563eb',
	magenta: '#9333ea',
	cyan: '#0891b2',
	white: '#6b7280',
	brightBlack: '#4b5563',
	brightRed: '#ef4444',
	brightGreen: '#22c55e',
	brightYellow: '#eab308',
	brightBlue: '#3b82f6',
	brightMagenta: '#a855f7',
	brightCyan: '#06b6d4',
	brightWhite: '#111827',
};

/** One PTY session: owns its xterm + socket; hidden tabs keep streaming. */
export function ShellSession({
	visible,
	enabled,
	targetContainer,
	shell,
	isRemoteServer,
	hostMode,
	serverId,
	onStatus,
}: {
	visible: boolean;
	enabled: boolean;
	targetContainer: string;
	shell: string;
	isRemoteServer: boolean;
	hostMode: boolean;
	serverId?: number;
	onStatus: (status: ShellStatus) => void;
}) {
	const containerRef = useRef<HTMLDivElement>(null);
	const fitRef = useRef<FitAddon | null>(null);
	const [termInstance, setTermInstance] = useState<Xterm | null>(null);
	const {theme} = useTheme();
	const xtermTheme = useMemo(
		() => (theme === 'dark' ? DARK_TERMINAL_THEME : LIGHT_TERMINAL_THEME),
		[theme],
	);

	// xterm lifecycle — created when enabled, disposed when hidden.
	useEffect(() => {
		if (!enabled || !containerRef.current) return;
		const host = containerRef.current;
		host.innerHTML = '';
		const term = new Xterm({
			cursorBlink: true,
			lineHeight: 1.35,
			fontSize: 13,
			fontFamily:
				'var(--font-mono, ui-monospace), Menlo, Monaco, Consolas, monospace',
			theme: xtermTheme,
		});
		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);
		term.open(host);
		fitRef.current = fitAddon;
		// ResizeObserver → fit: reflows on expand/collapse and container growth.
		const observer = new ResizeObserver(() => {
			try {
				fitAddon.fit();
			} catch {}
		});
		observer.observe(host);
		setTermInstance(term);
		return () => {
			observer.disconnect();
			term.dispose();
			setTermInstance(null);
			fitRef.current = null;
		};
		// xtermTheme is read only for the initial paint; live swaps happen in
		// the theme effect below — recreating here would drop the PTY session.
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [enabled]);

	// Repaint with the active palette whenever the app theme flips.
	useEffect(() => {
		if (termInstance) termInstance.options.theme = xtermTheme;
	}, [termInstance, xtermTheme]);

	// Re-fit when the tab becomes visible again (was display:none).
	useEffect(() => {
		if (!visible) return;
		const t = setTimeout(() => {
			try {
				fitRef.current?.fit();
			} catch {}
		}, 60);
		return () => clearTimeout(t);
	}, [visible]);

	const {status} = useTerminalSocket({
		isOpen: enabled,
		targetContainer,
		shell,
		isRemoteServer,
		hostMode,
		serverId,
		termInstance,
	});

	useEffect(() => {
		onStatus(status);
	}, [status, onStatus]);

	return (
		<div
			ref={containerRef}
			className="h-full w-full"
			style={{
				background: xtermTheme.background,
				...(visible ? undefined : {display: 'none'}),
			}}
		/>
	);
}
