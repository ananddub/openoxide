import { useEffect, useRef, useState, useMemo } from 'react';
import { createPortal } from 'react-dom';
import { Terminal as TerminalIcon, X, Box } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '#/components/ui/select';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { useTerminalSocket } from '#/hooks/terminal/use-terminal-socket';
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
	} catch (e) {
		console.warn('[extractServicesFromYaml] Error parsing YAML:', e);
	}
	return [];
};

const CONTROL_KEY_MAP: Record<string, string> = {
	l: '\x0c', c: '\x03', d: '\x04', z: '\x1a', u: '\x15', a: '\x01', e: '\x05', k: '\x0b', w: '\x17',
};

export function TerminalModal({ app, open, onClose }: TerminalModalProps) {
	const [shell, setShell] = useState<'sh' | 'bash'>('sh');
	const [selectedService, setSelectedService] = useState('');
	const termRef = useRef<HTMLDivElement>(null);
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

	const { status, socketRef } = useTerminalSocket({
		isOpen: open,
		targetContainer,
		shell,
		isRemoteServer,
		serverId,
		termRef: termInstanceRef,
	});

	useEffect(() => {
		if (!open) return;
		let term: Terminal | null = null;
		let fitAddon: FitAddon | null = null;

		const timer = setTimeout(() => {
			if (!termRef.current) return;
			termRef.current.innerHTML = '';
			term = new Terminal({
				cursorBlink: true, lineHeight: 1.4, convertEol: true, fontSize: 13,
				fontFamily: 'Menlo, Monaco, "Courier New", monospace',
				theme: { background: '#09090b', foreground: '#f4f4f5', cursor: '#3b82f6' },
			});
			termInstanceRef.current = term;

			fitAddon = new FitAddon();
			term.loadAddon(fitAddon);
			term.open(termRef.current);
			try { fitAddon.fit(); } catch (_) {}

			setTimeout(() => {
				term?.focus();
				const helper = termRef.current?.querySelector('textarea');
				if (helper) helper.focus();
			}, 50);

			const handlePaste = (e: ClipboardEvent) => {
				const text = e.clipboardData?.getData('text');
				if (text && socketRef.current?.connected) {
					socketRef.current.emit('input', { data: text });
				}
			};
			const el = termRef.current;
			if (el) el.addEventListener('paste', handlePaste);

			term.attachCustomKeyEventHandler((event: KeyboardEvent) => {
				if (event.ctrlKey || event.metaKey) {
					const k = event.key.toLowerCase();
					if (k === 'c' && term?.hasSelection()) {
						if (event.type === 'keydown') navigator.clipboard.writeText(term.getSelection());
						event.preventDefault(); return false;
					}
					if (k === 'v') {
						if (event.type === 'keydown') {
							navigator.clipboard.readText().then(text => {
								if (text && socketRef.current?.connected) socketRef.current.emit('input', { data: text });
							}).catch(() => {});
						}
						event.preventDefault(); return false;
					}
					if (CONTROL_KEY_MAP[k]) {
						if (event.type === 'keydown' && socketRef.current?.connected) {
							if (k === 'l') { term?.clear(); socketRef.current.emit('input', { data: 'clear\r' }); }
							else { socketRef.current.emit('input', { data: CONTROL_KEY_MAP[k] }); }
						}
						event.preventDefault(); return false;
					}
				}
				return true;
			});

			term.onData(data => { if (socketRef.current?.connected) socketRef.current.emit('input', { data }); });
			term.onResize(({ cols, rows }) => { if (socketRef.current?.connected) socketRef.current.emit('resize', { cols, rows }); });
		}, 100);

		const handleWindowResize = () => { try { fitAddon?.fit(); } catch (_) {} };
		window.addEventListener('resize', handleWindowResize);

		return () => {
			clearTimeout(timer);
			window.removeEventListener('resize', handleWindowResize);
			term?.dispose();
			termInstanceRef.current = null;
		};
	}, [open, targetContainer]);

	if (!open) return null;

	return createPortal(
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-xs p-4 animate-in fade-in duration-150">
			<div className="flex flex-col w-full max-w-5xl h-[85vh] bg-[#09090b] border border-border/80 rounded-xl shadow-2xl overflow-hidden">
				{/* Modal Header */}
				<div className="flex items-center justify-between px-4 py-3 border-b border-border/60 bg-muted/20 shrink-0">
					<div className="flex items-center gap-3">
						<TerminalIcon className="size-5 text-primary shrink-0" />
						<div>
							<h3 className="text-sm font-semibold text-foreground flex items-center gap-2">
								Terminal Stream
								<span className={`inline-flex items-center px-2 py-0.5 rounded text-[10px] font-mono font-medium border ${
									status === 'connected' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' :
									status === 'connecting' ? 'bg-amber-500/10 text-amber-400 border-amber-500/20' :
									'bg-rose-500/10 text-rose-400 border-rose-500/20'
								}`}>
									{status}
								</span>
							</h3>
							<p className="text-xs text-muted-foreground font-mono truncate max-w-md">
								Container: <span className="text-foreground font-semibold">{targetContainer}</span>
							</p>
						</div>
					</div>

					<div className="flex items-center gap-3">
						{isCompose && servicesList.length > 1 && (
							<div className="flex items-center gap-1.5">
								<Box className="size-3.5 text-muted-foreground shrink-0" />
								<Select value={targetContainer} onValueChange={v => v && setSelectedService(v)}>
									<SelectTrigger className="h-8 text-xs font-mono bg-card border-border/60 w-[140px]">
										<SelectValue placeholder="Service" />
									</SelectTrigger>
									<SelectContent className="bg-card border-border text-xs">
										{servicesList.map(s => <SelectItem key={s} value={s} className="text-xs font-mono">{s}</SelectItem>)}
									</SelectContent>
								</Select>
							</div>
						)}

						<Select value={shell} onValueChange={v => v && setShell(v as 'sh' | 'bash')}>
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
					const helper = termRef.current?.querySelector('textarea');
					if (helper) helper.focus();
				}}>
					<div ref={termRef} className="w-full h-full text-left font-mono" />
				</div>
			</div>
		</div>,
		document.body
	);
}
