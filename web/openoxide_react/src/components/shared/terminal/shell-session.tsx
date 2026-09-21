import {useEffect, useRef, useState} from 'react';
import {Terminal as Xterm} from '@xterm/xterm';
import {FitAddon} from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import {useTerminalSocket} from '#/hooks/terminal/use-terminal-socket';
import type {ShellStatus} from './types';

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
			theme: {
				background: 'transparent',
				foreground: 'var(--terminal-fg, #e5e5e5)',
				cursor: '#22c55e',
			},
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
	}, [enabled]);

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
			style={visible ? undefined : {display: 'none'}}
		/>
	);
}
