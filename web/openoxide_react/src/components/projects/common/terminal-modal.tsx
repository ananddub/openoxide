import { useEffect, useRef, useState, useMemo } from 'react';
import { createPortal } from 'react-dom';
import { Terminal as TerminalIcon, X, Box, Server as ServerIcon } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '#/components/ui/select';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { io, type Socket } from 'socket.io-client';
import { load as yamlLoad } from 'js-yaml';

interface TerminalModalProps {
	app: any;
	open: boolean;
	onClose: () => void;
}

export const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr || !yamlStr.trim()) return [];
	try {
		let cleanYaml = yamlStr.trim();
		if (cleanYaml.startsWith('```')) {
			cleanYaml = cleanYaml.replace(/^```[a-zA-Z]*\n?/, '').replace(/\n?```$/, '').trim();
		}
		const doc: any = yamlLoad(cleanYaml);
		if (doc && typeof doc === 'object' && doc.services && typeof doc.services === 'object' && !Array.isArray(doc.services)) {
			return Object.keys(doc.services);
		}
	} catch {}
	return [];
};

const CONTROL_KEY_MAP: Record<string, string> = {
	l: '\x0c', c: '\x03', d: '\x04', z: '\x1a', u: '\x15', a: '\x01', e: '\x05', k: '\x0b', w: '\x17',
};

function getSocketBaseUrl(): string {
	if (import.meta.env.VITE_SOCKET_URL) return import.meta.env.VITE_SOCKET_URL;
	if (import.meta.env.DEV) return 'http://127.0.0.1:4000';
	return '';
}

export function TerminalModal({ app, open, onClose }: TerminalModalProps) {
	// Default shell set to 'bash' as requested for remote & docker terminal sessions
	const [shell, setShell] = useState<'sh' | 'bash'>('bash');
	const [selectedService, setSelectedService] = useState('');
	const [status, setStatus] = useState<'connecting' | 'connected' | 'disconnected' | 'error'>('disconnected');
	const termRef = useRef<HTMLDivElement>(null);
	const socketRef = useRef<Socket | null>(null);
	const termInstanceRef = useRef<Terminal | null>(null);

	const availableServices = useMemo(() => extractServicesFromYaml(app?.compose_file), [app?.compose_file]);
	const isCompose = app?.compose_status !== undefined || app?.compose_type !== undefined || app?.compose_file !== undefined;

	const defaultContainer = useMemo(() => {
		if (availableServices.length > 0) return availableServices[0];
		if (isCompose) return 'app';
		const name = app?.app_name || app?.appName || app?.name || 'app';
		const isDatabase = Boolean(app?.kind || app?.database_kind || app?.database_name || app?.database_password);
		return isDatabase && !name.endsWith('_db') ? `${name}_db` : name;
	}, [availableServices, isCompose, app]);

	const targetContainer = selectedService || defaultContainer;
	const servicesList = availableServices.length > 0 ? availableServices : ['app'];
	const isRemoteServer = Boolean(app?.isRemoteServer);
	const serverId = app?.server_id || app?.serverId;

	// 1. Primary Socket & Xterm lifecycle: Runs ONLY when `open` changes
	useEffect(() => {
		if (!open) {
			if (socketRef.current) {
				socketRef.current.disconnect();
				socketRef.current = null;
			}
			if (termInstanceRef.current) {
				termInstanceRef.current.dispose();
				termInstanceRef.current = null;
			}
			setStatus('disconnected');
			return;
		}

		if (!termRef.current) return;
		termRef.current.innerHTML = '';

		// Full Vibrant 24-bit TrueColor ANSI Theme Palette matching Alacritty / VS Code Pro
		const term = new Terminal({
			cursorBlink: true,
			lineHeight: 1.35,
			convertEol: true,
			fontSize: 13,
			fontFamily: 'Menlo, Monaco, "Consolas", "Courier New", monospace',
			allowProposedApi: true,
			theme: {
				background: '#09090b',
				foreground: '#f4f4f5',
				cursor: '#3b82f6',
				cursorAccent: '#09090b',
				selectionBackground: '#3f3f46',
				selectionForeground: '#ffffff',
				black: '#18181b',
				red: '#f43f5e',
				green: '#10b981',
				yellow: '#f59e0b',
				blue: '#3b82f6',
				magenta: '#d946ef',
				cyan: '#06b6d4',
				white: '#e4e4e7',
				brightBlack: '#71717a',
				brightRed: '#fb7185',
				brightGreen: '#34d399',
				brightYellow: '#fbbf24',
				brightBlue: '#60a5fa',
				brightMagenta: '#e879f9',
				brightCyan: '#22d3ee',
				brightWhite: '#ffffff',
			},
		});
		termInstanceRef.current = term;

		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);
		term.open(termRef.current);
		try { fitAddon.fit(); } catch (_) {}

		setStatus('connecting');
		if (isRemoteServer) {
			term.writeln(`\x1b[33mConnecting to Remote Server [${targetContainer}] via SSH...\x1b[0m\r\n`);
		} else {
			term.writeln(`\x1b[33mConnecting to Docker Container [${targetContainer}]...\x1b[0m\r\n`);
		}

		let token: string | undefined;
		try {
			token = JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token;
		} catch {}

		const socketUrl = `${getSocketBaseUrl()}/terminal`;
		const socket = io(socketUrl, {
			path: '/socket.io',
			transports: ['websocket'],
			reconnection: true,
			reconnectionAttempts: 10,
			auth: (cb) => {
				let t = token;
				try { t = JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token; } catch {}
				cb({ token: t });
			},
		});
		socketRef.current = socket;

		socket.on('connect', () => {
			setStatus('connected');
			if (isRemoteServer) {
				term.writeln(`\x1b[32mSocket connected. Launching SSH shell [${shell}] on Remote Server [${targetContainer}]...\x1b[0m\r\n`);
				socket.emit('server:start', { server_id: serverId, command: shell });
			} else {
				term.writeln(`\x1b[32mSocket connected. Starting shell [${shell}] on Container [${targetContainer}]...\x1b[0m\r\n`);
				socket.emit('docker:start', { container: targetContainer, shell });
			}
		});

		socket.on('started', (data: { kind?: string }) => {
			const label = data?.kind === 'remote-server' ? 'SSH Remote Server' : 'Docker Container';
			term.writeln(`\x1b[32mTerminal session started (${label}). Type commands below:\x1b[0m\r\n`);
			term.focus();
		});

		socket.on('output', (evt: { data: string }) => {
			if (evt?.data) term.write(evt.data);
		});

		socket.on('error', (err: unknown) => {
			const msg = typeof err === 'string' ? err : (err as { message?: string })?.message || 'Error';
			term.writeln(`\r\n\x1b[31mError: ${msg}\x1b[0m\r\n`);
			setStatus('error');
		});

		socket.on('exit', (evt: { code: number }) => {
			term.writeln(`\r\n\x1b[33mProcess exited with code ${evt?.code ?? 0}\x1b[0m\r\n`);
			setStatus('disconnected');
		});

		socket.on('disconnect', (reason) => {
			if (socketRef.current !== socket) return;
			setStatus('disconnected');
			if (reason !== 'io client disconnect') {
				term.writeln(`\r\n\x1b[31mSocket disconnected (${reason}). Reconnecting...\x1b[0m\r\n`);
			}
		});

		term.onData((data) => { if (socket.connected) socket.emit('input', { data }); });
		term.onResize(({ cols, rows }) => { if (socket.connected) socket.emit('resize', { cols, rows }); });

		term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
			if (event.ctrlKey || event.metaKey) {
				const k = event.key.toLowerCase();
				if (k === 'c' && term.hasSelection()) {
					if (event.type === 'keydown') navigator.clipboard.writeText(term.getSelection());
					event.preventDefault(); return false;
				}
				if (k === 'v') {
					if (event.type === 'keydown') {
						navigator.clipboard.readText().then((text) => {
							if (text && socket.connected) socket.emit('input', { data: text });
						}).catch(() => {});
					}
					event.preventDefault(); return false;
				}
				if (CONTROL_KEY_MAP[k]) {
					if (event.type === 'keydown' && socket.connected) {
						if (k === 'l') { term.clear(); socket.emit('input', { data: 'clear\r' }); }
						else { socket.emit('input', { data: CONTROL_KEY_MAP[k] }); }
					}
					event.preventDefault(); return false;
				}
			}
			return true;
		});

		const handleWindowResize = () => { try { fitAddon.fit(); } catch (_) {} };
		window.addEventListener('resize', handleWindowResize);

		return () => {
			window.removeEventListener('resize', handleWindowResize);
			if (socketRef.current) {
				socketRef.current.disconnect();
				socketRef.current = null;
			}
			if (termInstanceRef.current) {
				termInstanceRef.current.dispose();
				termInstanceRef.current = null;
			}
		};
	}, [open]);

	// 2. Dynamic Shell / Target Container Switch: Emits docker:start on existing live socket without tear-down
	const isFirstMountRef = useRef(true);
	useEffect(() => {
		if (isFirstMountRef.current) {
			isFirstMountRef.current = false;
			return;
		}
		if (!open || !socketRef.current?.connected || !termInstanceRef.current) return;

		const term = termInstanceRef.current;
		if (isRemoteServer) {
			term.writeln(`\r\n\x1b[33mSwitching shell to [${shell}] on Remote Server [${targetContainer}]...\x1b[0m\r\n`);
		} else {
			term.writeln(`\r\n\x1b[33mSwitching shell to [${shell}] on Container [${targetContainer}]...\x1b[0m\r\n`);
		}
		setStatus('connecting');

		if (isRemoteServer && serverId) {
			socketRef.current.emit('server:start', { server_id: serverId, command: shell });
		} else {
			socketRef.current.emit('docker:start', { container: targetContainer, shell });
		}
	}, [targetContainer, shell]);

	if (!open) return null;

	return createPortal(
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-xs p-4 animate-in fade-in duration-150">
			<div className="flex flex-col w-full max-w-5xl h-[85vh] bg-[#09090b] border border-border/80 rounded-xl shadow-2xl overflow-hidden">
				{/* Modal Header */}
				<div className="flex items-center justify-between px-4 py-3 border-b border-border/60 bg-muted/20 shrink-0">
					<div className="flex items-center gap-3">
						{isRemoteServer ? (
							<ServerIcon className="size-5 text-primary shrink-0" />
						) : (
							<TerminalIcon className="size-5 text-primary shrink-0" />
						)}
						<div>
							<h3 className="text-sm font-semibold text-foreground flex items-center gap-2">
								{isRemoteServer ? 'Remote Server SSH Terminal' : 'Container Terminal Stream'}
								<span className={`inline-flex items-center px-2 py-0.5 rounded text-[10px] font-mono font-medium border ${
									status === 'connected' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' :
									status === 'connecting' ? 'bg-amber-500/10 text-amber-400 border-amber-500/20' :
									'bg-rose-500/10 text-rose-400 border-rose-500/20'
								}`}>
									{status}
								</span>
							</h3>
							<p className="text-xs text-muted-foreground font-mono truncate max-w-md">
								{isRemoteServer ? 'Server Host: ' : 'Container: '}
								<span className="text-foreground font-semibold">{targetContainer}</span>
							</p>
						</div>
					</div>

					<div className="flex items-center gap-3">
						{isCompose && servicesList.length > 1 && (
							<div className="flex items-center gap-1.5">
								<Box className="size-3.5 text-muted-foreground shrink-0" />
								<Select value={targetContainer} onValueChange={(v) => v && setSelectedService(v)}>
									<SelectTrigger className="h-8 text-xs font-mono bg-card border-border/60 w-[140px]">
										<SelectValue placeholder="Service" />
									</SelectTrigger>
									<SelectContent className="bg-card border-border text-xs">
										{servicesList.map((s) => <SelectItem key={s} value={s} className="text-xs font-mono">{s}</SelectItem>)}
									</SelectContent>
								</Select>
							</div>
						)}

						<Select value={shell} onValueChange={(v) => v && setShell(v as 'sh' | 'bash')}>
							<SelectTrigger className="h-8 text-xs font-mono bg-card border-border/60 w-[90px]">
								<SelectValue placeholder="Shell" />
							</SelectTrigger>
							<SelectContent className="bg-card border-border text-xs">
								<SelectItem value="sh" className="text-xs font-mono">sh</SelectItem>
								<SelectItem value="bash" className="text-xs font-mono">bash</SelectItem>
							</SelectContent>
						</Select>

						<Button variant="ghost" size="icon" onClick={onClose} className="size-8 text-muted-foreground hover:text-foreground cursor-pointer">
							<X className="size-4" />
						</Button>
					</div>
				</div>

				{/* Xterm.js Canvas Box */}
				<div className="flex-1 p-3 bg-[#09090b] relative overflow-hidden min-h-0 cursor-text" onClick={() => {
					termInstanceRef.current?.focus();
				}}>
					<div ref={termRef} className="w-full h-full text-left font-mono" />
				</div>
			</div>
		</div>,
		document.body
	);
}
