import { useEffect, useRef, useState, useMemo } from 'react';
import { createPortal } from 'react-dom';
import { Terminal as TerminalIcon, X, Box, Server as ServerIcon } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '#/components/ui/select';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { load as yamlLoad } from 'js-yaml';
import { useTerminalSocket } from '#/hooks/terminal/use-terminal-socket';

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

export function TerminalModal({ app, open, onClose }: TerminalModalProps) {
	const [shell, setShell] = useState<'sh' | 'bash'>('bash');
	const [selectedService, setSelectedService] = useState('');
	const [termInstance, setTermInstance] = useState<Terminal | null>(null);
	const termRef = useRef<HTMLDivElement>(null);

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
	const serverId = app?.server_id || app?.serverId || (isRemoteServer ? app?.id : undefined);

	// Initialize Xterm instance and FitAddon
	useEffect(() => {
		if (!open) {
			if (termInstance) {
				termInstance.dispose();
				setTermInstance(null);
			}
			return;
		}

		if (!termRef.current) return;
		termRef.current.innerHTML = '';

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

		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);
		term.open(termRef.current);
		try { fitAddon.fit(); } catch (_) {}
		setTermInstance(term);

		const handleWindowResize = () => { try { fitAddon.fit(); } catch (_) {} };
		window.addEventListener('resize', handleWindowResize);

		return () => {
			window.removeEventListener('resize', handleWindowResize);
			term.dispose();
			setTermInstance(null);
		};
	}, [open]);

	// Reusable Terminal Socket Hook
	const { status, activeHostIp } = useTerminalSocket({
		isOpen: open,
		targetContainer,
		shell,
		isRemoteServer,
		serverId,
		termInstance,
	});

	if (!open) return null;

	const displayHost = activeHostIp || targetContainer;

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
							<p className="text-xs text-muted-foreground font-mono mt-0.5">
								Target: <span className="text-foreground font-semibold">{displayHost}</span>
							</p>
						</div>
					</div>

					<div className="flex items-center gap-3">
						{!isRemoteServer && isCompose && servicesList.length > 1 && (
							<div className="flex items-center gap-1.5">
								<Box className="size-3.5 text-muted-foreground" />
								<Select value={targetContainer} onValueChange={setSelectedService}>
									<SelectTrigger className="h-8 text-xs w-[140px] bg-background/50 border-border/80">
										<SelectValue placeholder="Select Service" />
									</SelectTrigger>
									<SelectContent>
										{servicesList.map((svc) => (
											<SelectItem key={svc} value={svc} className="text-xs">
												{svc}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</div>
						)}

						<Select value={shell} onValueChange={(val) => setShell(val as 'sh' | 'bash')}>
							<SelectTrigger className="h-8 text-xs w-[90px] bg-background/50 border-border/80 font-mono">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="bash" className="text-xs font-mono">bash</SelectItem>
								<SelectItem value="sh" className="text-xs font-mono">sh</SelectItem>
							</SelectContent>
						</Select>

						<Button
							variant="ghost"
							size="icon"
							className="size-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-accent"
							onClick={onClose}
						>
							<X className="size-4" />
						</Button>
					</div>
				</div>

				{/* Xterm Terminal Container */}
				<div className="flex-1 bg-[#09090b] p-3 overflow-hidden relative">
					<div ref={termRef} className="w-full h-full text-left" />
				</div>
			</div>
		</div>,
		document.body
	);
}
