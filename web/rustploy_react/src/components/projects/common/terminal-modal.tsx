import {useEffect, useRef, useState, useMemo} from 'react';
import {Terminal as TerminalIcon, X, Box} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {Terminal} from '@xterm/xterm';
import {FitAddon} from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import {io, Socket} from 'socket.io-client';

interface TerminalModalProps {
	app: any;
	open: boolean;
	onClose: () => void;
}

const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const services: string[] = [];
	let inServices = false;
	let indentLevel = 0;

	for (const line of yamlStr.split('\n')) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;
		const indent = line.search(/\S/);
		const text = trimmed.trim();

		if (text.startsWith('services:')) {
			inServices = true;
			indentLevel = indent;
			continue;
		}
		if (inServices) {
			if (indent <= indentLevel && text.endsWith(':') && !text.startsWith('-')) inServices = false;
			else if (indent > indentLevel && text.endsWith(':') && !text.includes(' ')) {
				const srv = text.slice(0, -1).trim();
				if (srv && !services.includes(srv)) services.push(srv);
			}
		}
	}
	return services;
};

const CONTROL_KEY_MAP: Record<string, string> = {
	l: '\x0c', // Clear screen (Ctrl+L)
	c: '\x03', // SIGINT (Ctrl+C)
	d: '\x04', // EOF (Ctrl+D)
	z: '\x1a', // SIGTSTP (Ctrl+Z)
	u: '\x15', // Clear line (Ctrl+U)
	a: '\x01', // Beginning of line (Ctrl+A)
	e: '\x05', // End of line (Ctrl+E)
	k: '\x0b', // Kill line (Ctrl+K)
	w: '\x17', // Erase word (Ctrl+W)
};

export function TerminalModal({app, open, onClose}: TerminalModalProps) {
	const [shell, setShell] = useState<'sh' | 'bash'>('sh');
	const [selectedService, setSelectedService] = useState('');
	const [status, setStatus] = useState<'connecting' | 'connected' | 'disconnected' | 'error'>('connecting');
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
		if (isDatabase && !name.endsWith('_db')) {
			return `${name}_db`;
		}
		return name;
	}, [availableServices, isCompose, app]);

	const targetContainer = selectedService || defaultContainer;
	const servicesList = availableServices.length > 0 ? availableServices : ['app'];

	const isRemoteServer = Boolean(app?.isRemoteServer);
	const serverId = app?.server_id || app?.serverId;

	useEffect(() => {
		if (!open) return;
		let term: Terminal | null = null;
		let fitAddon: FitAddon | null = null;

		const timer = setTimeout(() => {
			if (!termRef.current) return;
			termRef.current.innerHTML = '';
			term = new Terminal({
				cursorBlink: true,
				lineHeight: 1.4,
				convertEol: true,
				fontSize: 13,
				fontFamily: 'Menlo, Monaco, "Courier New", monospace',
				theme: {background: '#09090b', foreground: '#f4f4f5', cursor: '#3b82f6'},
			});
			termInstanceRef.current = term;

			fitAddon = new FitAddon();
			term.loadAddon(fitAddon);
			term.open(termRef.current);
			try { fitAddon.fit(); } catch (_) {}

			// Force focus on xterm helper textarea
			setTimeout(() => {
				term?.focus();
				const helper = termRef.current?.querySelector('textarea');
				if (helper) helper.focus();
			}, 50);

			// Clipboard paste event listener (Right-click paste & Ctrl+V)
			const handlePaste = (e: ClipboardEvent) => {
				const text = e.clipboardData?.getData('text');
				if (text && socketRef.current?.connected) {
					socketRef.current.emit('input', { data: text });
				}
			};
			const el = termRef.current;
			if (el) el.addEventListener('paste', handlePaste);

			// Custom key bindings (Control shortcuts, Copy, Paste, Ctrl+L hijack prevention)
			term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
				if (event.ctrlKey || event.metaKey) {
					const k = event.key.toLowerCase();
					if (k === 'c' && term?.hasSelection()) {
						if (event.type === 'keydown') {
							navigator.clipboard.writeText(term.getSelection());
						}
						event.preventDefault();
						return false;
					}
					if (k === 'v') {
						if (event.type === 'keydown') {
							navigator.clipboard.readText().then(text => {
								if (text && socketRef.current?.connected) {
									socketRef.current.emit('input', { data: text });
								}
							}).catch(() => {});
						}
						event.preventDefault();
						return false;
					}
					if (CONTROL_KEY_MAP[k]) {
						if (event.type === 'keydown' && socketRef.current?.connected) {
							if (k === 'l') {
								term?.clear();
								socketRef.current.emit('input', { data: 'clear\r' });
							} else {
								socketRef.current.emit('input', { data: CONTROL_KEY_MAP[k] });
							}
						}
						event.preventDefault();
						return false;
					}
				}
				return true;
			});

			setStatus('connecting');
			term.writeln(`\x1b[33mConnecting to container/host '${targetContainer}'...\x1b[0m\r\n`);

			const targetPort = window.location.port && window.location.port !== '4000' ? '4000' : window.location.port;
			const socketUrl = `${window.location.protocol}//${window.location.hostname}${targetPort ? `:${targetPort}` : ''}`;

			const socket = io(`${socketUrl}/terminal`, {path: '/socket.io', transports: ['websocket', 'polling']});
			socketRef.current = socket;

			socket.on('connect', () => {
				setStatus('connected');
				term?.writeln(`\x1b[32mSocket connected to ${socketUrl}. Starting shell [${shell}] on '${targetContainer}'...\x1b[0m\r\n`);
				if (isRemoteServer && serverId) {
					socket.emit('server:start', {server_id: serverId, command: shell});
				} else {
					socket.emit('docker:start', {container: targetContainer, shell});
				}
			});
			socket.on('started', (data: any) => {
				term?.writeln(`\x1b[32mTerminal session started (${data?.kind || 'docker'}). Type commands below:\x1b[0m\r\n`);
				term?.focus();
				const helper = termRef.current?.querySelector('textarea');
				if (helper) helper.focus();
			});
			socket.on('output', (evt: {data: string}) => { if (evt?.data) term?.write(evt.data); });
			socket.on('error', (err: any) => {
				term?.writeln(`\r\n\x1b[31mError: ${typeof err === 'string' ? err : err?.message || 'Error'}\x1b[0m\r\n`);
				setStatus('error');
			});
			socket.on('exit', (evt: {code: number}) => {
				term?.writeln(`\r\n\x1b[33mProcess exited with code ${evt?.code ?? 0}\x1b[0m\r\n`);
				setStatus('disconnected');
			});
			socket.on('disconnect', () => {
				setStatus('disconnected');
				term?.writeln('\r\n\x1b[31mSocket disconnected.\x1b[0m\r\n');
			});

			term.onData(data => {
				if (socketRef.current?.connected) {
					socketRef.current.emit('input', {data});
				}
			});
			term.onResize(({cols, rows}) => {
				if (socketRef.current?.connected) {
					socketRef.current.emit('resize', {cols, rows});
				}
			});

			return () => {
				if (el) el.removeEventListener('paste', handlePaste);
			};
		}, 100);

		const handleResize = () => { try { fitAddon?.fit(); } catch (_) {} };
		window.addEventListener('resize', handleResize);

		return () => {
			clearTimeout(timer);
			window.removeEventListener('resize', handleResize);
			if (socketRef.current) {
				socketRef.current.emit('stop');
				socketRef.current.disconnect();
			}
			if (term) term.dispose();
			termInstanceRef.current = null;
		};
	}, [open, shell, targetContainer, isRemoteServer, serverId]);

	if (!open) return null;

	const handleFocus = () => {
		termInstanceRef.current?.focus();
		const helper = termRef.current?.querySelector('textarea');
		if (helper) helper.focus();
	};

	return (
		<div className="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-4xl max-h-[85vh] h-[550px] flex flex-col overflow-hidden animate-in fade-in duration-150">
				<div className="flex items-center justify-between border-b border-border p-4 bg-card shrink-0">
					<div className="flex items-center gap-3">
						<TerminalIcon className="w-5 h-5 text-primary" />
						<div>
							<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
								Docker / SSH Terminal
								<span className={`text-[10px] font-semibold px-2 py-0.5 rounded-full border ${status === 'connected' ? 'text-emerald-500 bg-emerald-500/10 border-emerald-500/20' : status === 'connecting' ? 'text-amber-500 bg-amber-500/10 border-amber-500/20 animate-pulse' : 'text-rose-500 bg-rose-500/10 border-rose-500/20'}`}>{status}</span>
							</h3>
							<p className="text-[11px] text-muted-foreground font-mono">{targetContainer}</p>
						</div>
					</div>

					<div className="flex items-center gap-3">
						{isCompose && (
							<Select value={targetContainer} onValueChange={v => setSelectedService(v)}>
								<SelectTrigger size="sm" className="h-8 text-xs font-bold bg-muted/50 border-border hover:bg-muted/80">
									<Box className="w-3.5 h-3.5 text-primary shrink-0 mr-1" />
									<SelectValue placeholder="Select Service" />
								</SelectTrigger>
								<SelectContent>
									{servicesList.map(service => (
										<SelectItem key={service} value={service}><span className="font-semibold">Service: {service}</span></SelectItem>
									))}
								</SelectContent>
							</Select>
						)}
						<div className="flex items-center gap-1 bg-muted p-1 rounded-lg border border-border">
							<Button variant={shell === 'sh' ? 'secondary' : 'ghost'} size="xs" onClick={() => setShell('sh')}>/bin/sh</Button>
							<Button variant={shell === 'bash' ? 'secondary' : 'ghost'} size="xs" onClick={() => setShell('bash')}>bash</Button>
						</div>
						<Button variant="ghost" size="icon" onClick={onClose} className="h-8 w-8 text-muted-foreground hover:text-foreground"><X className="w-4 h-4" /></Button>
					</div>
				</div>

				<div className="flex-1 bg-[#09090b] p-3 overflow-hidden relative cursor-text" onClick={handleFocus}>
					<div ref={termRef} tabIndex={0} onFocus={handleFocus} className="absolute inset-2 outline-none" />
				</div>
			</div>
		</div>
	);
}
