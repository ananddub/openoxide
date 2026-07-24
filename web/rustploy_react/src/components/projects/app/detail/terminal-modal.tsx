import {useEffect, useRef, useState} from 'react';
import {Terminal as TerminalIcon, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Terminal} from '@xterm/xterm';
import {FitAddon} from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import {io, Socket} from 'socket.io-client';

interface TerminalModalProps {
	app: any;
	open: boolean;
	onClose: () => void;
}

export function TerminalModal({app, open, onClose}: TerminalModalProps) {
	const [shell, setShell] = useState<'sh' | 'bash'>('sh');
	const [status, setStatus] = useState<'connecting' | 'connected' | 'disconnected' | 'error'>('connecting');
	const termRef = useRef<HTMLDivElement>(null);
	const socketRef = useRef<Socket | null>(null);

	const containerName = app.app_name || app.appName || app.name || 'app';

	useEffect(() => {
		if (!open || !termRef.current) return;

		termRef.current.innerHTML = '';
		const term = new Terminal({
			cursorBlink: true,
			lineHeight: 1.4,
			convertEol: true,
			fontSize: 13,
			fontFamily: 'Menlo, Monaco, "Courier New", monospace',
			theme: {
				background: '#09090b',
				foreground: '#f4f4f5',
				cursor: '#3b82f6',
			},
		});

		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);
		term.open(termRef.current);
		fitAddon.fit();

		setStatus('connecting');
		term.writeln(`\x1b[33mConnecting to container '${containerName}'...\x1b[0m\r\n`);

		const socket = io('/terminal', {
			path: '/socket.io',
			transports: ['websocket', 'polling'],
		});
		socketRef.current = socket;

		socket.on('connect', () => {
			setStatus('connected');
			term.writeln(`\x1b[32mSocket connected. Starting shell [${shell}]...\x1b[0m\r\n`);
			socket.emit('docker:start', {
				container: containerName,
				shell,
			});
		});

		socket.on('started', (data: any) => {
			term.writeln(`\x1b[32mTerminal session started (${data?.kind || 'docker'}). Type commands below:\x1b[0m\r\n`);
			socket.emit('input', {data: 'pwd\n'});
		});

		socket.on('output', (evt: {stream: string; data: string}) => {
			if (evt?.data) {
				term.write(evt.data);
			}
		});

		socket.on('error', (err: {message: string} | string) => {
			const msg = typeof err === 'string' ? err : err?.message || 'Terminal error';
			term.writeln(`\r\n\x1b[31mError: ${msg}\x1b[0m\r\n`);
			setStatus('error');
		});

		socket.on('exit', (evt: {code: number}) => {
			term.writeln(`\r\n\x1b[33mProcess exited with code ${evt?.code ?? 0}\x1b[0m\r\n`);
			setStatus('disconnected');
		});

		socket.on('disconnect', () => {
			setStatus('disconnected');
			term.writeln('\r\n\x1b[31mSocket disconnected.\x1b[0m\r\n');
		});

		term.onData(data => {
			if (socket.connected) {
				socket.emit('input', {data});
			}
		});

		term.onResize(({cols, rows}) => {
			if (socket.connected) {
				socket.emit('resize', {cols, rows});
			}
		});

		const handleResize = () => {
			fitAddon.fit();
		};
		window.addEventListener('resize', handleResize);

		return () => {
			window.removeEventListener('resize', handleResize);
			if (socket.connected) {
				socket.emit('stop');
				socket.disconnect();
			}
			term.dispose();
		};
	}, [open, shell, containerName]);

	if (!open) return null;

	return (
		<div className="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-4xl max-h-[85vh] flex flex-col overflow-hidden animate-in fade-in duration-150">
				{/* Modal Header */}
				<div className="flex items-center justify-between border-b border-border p-4 bg-card">
					<div className="flex items-center gap-3">
						<TerminalIcon className="w-5 h-5 text-primary" />
						<div>
							<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
								Docker Terminal
								<span className={`text-[10px] font-semibold px-2 py-0.5 rounded-full border ${
									status === 'connected'
										? 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20'
										: status === 'connecting'
										? 'text-amber-500 bg-amber-500/10 border-amber-500/20 animate-pulse'
										: 'text-rose-500 bg-rose-500/10 border-rose-500/20'
								}`}>
									{status}
								</span>
							</h3>
							<p className="text-[11px] text-muted-foreground font-mono">{containerName}</p>
						</div>
					</div>

					<div className="flex items-center gap-2">
						{/* Shell Switcher */}
						<div className="flex items-center gap-1 bg-muted p-1 rounded-lg border border-border">
							<button
								type="button"
								onClick={() => setShell('sh')}
								className={`px-2.5 py-1 text-xs font-semibold rounded-md transition-colors ${shell === 'sh' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}`}
							>
								/bin/sh
							</button>
							<button
								type="button"
								onClick={() => setShell('bash')}
								className={`px-2.5 py-1 text-xs font-semibold rounded-md transition-colors ${shell === 'bash' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}`}
							>
								bash
							</button>
						</div>

						<Button variant="ghost" size="icon" onClick={onClose} className="h-8 w-8 text-muted-foreground hover:text-foreground">
							<X className="w-4 h-4" />
						</Button>
					</div>
				</div>

				{/* Terminal Output Container */}
				<div className="flex-1 bg-[#09090b] p-3 min-h-[400px] overflow-hidden">
					<div ref={termRef} className="w-full h-full" />
				</div>
			</div>
		</div>
	);
}
