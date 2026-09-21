import {useCallback, useEffect, useMemo, useRef, useState} from 'react';
import {
	Box,
	Maximize2,
	Minimize2,
	Plus,
	Server as ServerIcon,
	Terminal as TerminalIcon,
	X,
} from 'lucide-react';
import {Terminal as Xterm} from '@xterm/xterm';
import {FitAddon} from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import {load as yamlLoad} from 'js-yaml';
import {cn} from '#/api/utils';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useTerminalSocket} from '#/hooks/terminal/use-terminal-socket';

/** Compose service names from a compose file (used by callers too). */
export const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr || !yamlStr.trim()) return [];
	try {
		const doc = yamlLoad(yamlStr) as {services?: Record<string, unknown>};
		return Object.keys(doc?.services ?? {});
	} catch {}
	return [];
};

type ShellStatus = 'connecting' | 'connected' | 'error' | 'disconnected';

interface ShellEntry {
	id: string;
	label: string;
	status: ShellStatus;
}

interface TerminalCardProps {
	/** Resource descriptor: container/app/compose payload, or a remote
	 *  server row with `{isRemoteServer: true, id}`. */
	app: {
		app_name?: string;
		appName?: string;
		name?: string;
		compose_file?: string;
		compose_status?: unknown;
		compose_type?: unknown;
		kind?: string;
		database_kind?: string;
		database_name?: string;
		database_password?: string;
		server_id?: number;
		serverId?: number;
		isRemoteServer?: boolean;
		id?: number;
	};
	/** Only mount the PTY while true (keeps sessions from idling). */
	enabled: boolean;
	onClose?: () => void;
	className?: string;
	/** Open straight into the host root shell (server detail terminal). */
	hostModeDefault?: boolean;
}

/** One PTY session: owns its xterm + socket; hidden tabs keep streaming. */
function ShellSession({
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

/**
 * OpenShip's terminal surface (terminal-tab + ServerTerminalTabs pattern):
 * a card with an icon-tile header (title + description + expand), a shell
 * tab strip below it (Shell 1/2/3, "+ New Shell", per-tab status dot), and
 * xterm bodies. All tabs stay mounted — hidden ones keep their stream alive
 * so scrollback keeps flowing (VSCode "tabs run in background").
 */
export function TerminalCard({
	app,
	enabled,
	onClose,
	className,
	hostModeDefault = false,
}: TerminalCardProps) {
	const [shell, setShell] = useState<'sh' | 'bash'>('bash');
	const [selectedService, setSelectedService] = useState('');
	const [hostMode, setHostMode] = useState(hostModeDefault);
	const [expanded, setExpanded] = useState(false);

	const counterRef = useRef(1);
	const [shells, setShells] = useState<ShellEntry[]>(() => [
		{id: 'shell-1', label: 'Shell 1', status: 'disconnected'},
	]);
	const [activeId, setActiveId] = useState('shell-1');

	const availableServices = useMemo(
		() => extractServicesFromYaml(app?.compose_file),
		[app?.compose_file],
	);
	const isCompose =
		app?.compose_status !== undefined ||
		app?.compose_type !== undefined ||
		app?.compose_file !== undefined;

	const defaultContainer = useMemo(() => {
		if (availableServices.length > 0) return availableServices[0];
		if (isCompose) return 'app';
		const name = app?.app_name || app?.appName || app?.name || 'app';
		const isDatabase = Boolean(
			app?.kind ||
				app?.database_kind ||
				app?.database_name ||
				app?.database_password,
		);
		return isDatabase && !name.endsWith('_db') ? `${name}_db` : name;
	}, [availableServices, isCompose, app]);

	const targetContainer = selectedService || defaultContainer;
	const servicesList =
		availableServices.length > 0 ? availableServices : ['app'];
	const isRemoteServer = Boolean(app?.isRemoteServer);
	const serverId =
		app?.server_id ||
		app?.serverId ||
		(isRemoteServer ? app?.id : undefined);

	const displayName = app?.name || app?.app_name || app?.appName || 'shell';
	const subtitle = isRemoteServer
		? `Interactive SSH shell on ${targetContainer}.`
		: hostMode
			? `Host root shell via the Agent on ${targetContainer}.`
			: `Container shell for ${targetContainer}.`;

	const setStatus = useCallback((id: string, status: ShellStatus) => {
		setShells(prev => {
			const current = prev.find(s => s.id === id);
			if (!current || current.status === status) return prev;
			return prev.map(s => (s.id === id ? {...s, status} : s));
		});
	}, []);

	const handleAdd = useCallback(() => {
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

	const handleClose = useCallback(
		(id: string) => {
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
		},
		[],
	);

	// Esc leaves fullscreen (bound only while expanded).
	useEffect(() => {
		if (!expanded) return;
		const onKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') setExpanded(false);
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, [expanded]);

	return (
		<div
			className={cn(
				expanded
					? 'fixed inset-0 z-50 flex bg-background/95 p-4 backdrop-blur-sm'
					: '',
				className,
			)}>
			<div
				className={cn(
					'flex flex-col overflow-hidden rounded-2xl bg-card ring-1 ring-border/50',
					expanded ? 'h-full w-full' : 'w-full',
				)}>
				{/* Header — icon tile + title/description + controls */}
				<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
					<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted ring-1 ring-border/50">
						<TerminalIcon className="size-[18px] text-muted-foreground" />
					</div>
					<div className="min-w-0 flex-1">
						<h2 className="truncate text-[15px] font-semibold text-foreground">
							Terminal{' '}
							<span className="text-muted-foreground">
								· {displayName}
							</span>
						</h2>
						<p className="truncate text-xs text-muted-foreground">
							{subtitle}
						</p>
					</div>
					<div className="flex shrink-0 items-center gap-2">
						{!isRemoteServer && (
							<button
								type="button"
								onClick={() => setHostMode(v => !v)}
								title={
									hostMode
										? 'Switch back to container shell'
										: 'Open a root shell on the host (via Agent)'
								}
								className={cn(
									'inline-flex h-8 shrink-0 cursor-pointer items-center gap-1.5 rounded-xl px-2.5 text-[12px] font-medium ring-1 transition-colors',
									hostMode
										? 'bg-emerald-500/15 text-emerald-500 ring-emerald-500/40'
										: 'text-muted-foreground ring-border/50 hover:bg-muted hover:text-foreground',
								)}>
								<ServerIcon className="size-3.5" />
								Host
							</button>
						)}
						{!isRemoteServer && !hostMode && isCompose && servicesList.length > 1 && (
							<Select
								value={targetContainer}
								onValueChange={val => setSelectedService(val ?? '')}>
								<SelectTrigger className="h-8 min-h-8 w-[130px] gap-1 rounded-xl px-2.5 text-[12px] shadow-none">
									<Box className="size-3.5 shrink-0 text-muted-foreground" />
									<SelectValue placeholder="Service" />
								</SelectTrigger>
								<SelectContent>
									{servicesList.map(svc => (
										<SelectItem key={svc} value={svc} className="text-xs">
											{svc}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						)}
						<Select
							value={shell}
							onValueChange={val => setShell(val as 'sh' | 'bash')}>
							<SelectTrigger className="h-8 min-h-8 w-[84px] rounded-xl px-2.5 font-mono text-[12px] shadow-none">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="bash" className="font-mono text-xs">
									bash
								</SelectItem>
								<SelectItem value="sh" className="font-mono text-xs">
									sh
								</SelectItem>
							</SelectContent>
						</Select>
						<button
							type="button"
							onClick={() => setExpanded(v => !v)}
							aria-label={expanded ? 'Collapse terminal' : 'Expand terminal'}
							title={expanded ? 'Collapse (Esc)' : 'Expand'}
							className="inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-xl text-muted-foreground ring-1 ring-border/50 transition-colors hover:bg-muted hover:text-foreground">
							{expanded ? (
								<Minimize2 className="size-[18px]" />
							) : (
								<Maximize2 className="size-[18px]" />
							)}
						</button>
						{onClose && (
							<button
								type="button"
								onClick={onClose}
								aria-label="Close terminal"
								title="Close"
								className="inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-xl text-muted-foreground ring-1 ring-border/50 transition-colors hover:bg-muted hover:text-foreground">
								<X className="size-[18px]" />
							</button>
						)}
					</div>
				</div>

				{/* Shell tab strip */}
				<div className="flex items-center gap-1.5 border-b border-border/50 bg-muted/20 px-2.5 py-2">
					<div className="flex min-w-0 flex-1 items-center gap-1 overflow-x-auto">
						{shells.map(entry => {
							const isActive = entry.id === activeId;
							return (
								<div
									key={entry.id}
									className={cn(
										'group inline-flex shrink-0 items-center gap-1.5 rounded-lg py-1.5 pe-1.5 ps-2.5 text-[12px] transition-colors',
										isActive
											? 'bg-card text-foreground shadow-sm ring-1 ring-border/60'
											: 'text-muted-foreground hover:bg-foreground/[0.05] hover:text-foreground',
									)}>
									<button
										type="button"
										onClick={() => setActiveId(entry.id)}
										className="inline-flex cursor-pointer items-center gap-1.5 font-medium">
										<span
											className={cn(
												'size-1.5 rounded-full transition-colors',
												entry.status === 'connected'
													? 'bg-emerald-500'
													: entry.status === 'connecting'
														? 'bg-amber-500'
														: entry.status === 'error'
															? 'bg-rose-500'
															: 'bg-muted-foreground/40',
											)}
										/>
										{entry.label}
									</button>
									<button
										type="button"
										onClick={e => {
											e.stopPropagation();
											handleClose(entry.id);
										}}
										aria-label={`Close ${entry.label}`}
										className={cn(
											'cursor-pointer rounded-md p-0.5 text-muted-foreground transition-all hover:bg-foreground/10 hover:text-foreground',
											isActive
												? 'opacity-70 hover:opacity-100'
												: 'opacity-0 group-hover:opacity-70',
										)}>
										<X className="size-3" />
									</button>
								</div>
							);
						})}
					</div>
					<button
						type="button"
						onClick={handleAdd}
						disabled={shells.length >= 3}
						title={
							shells.length >= 3
								? 'Maximum of 3 shells per terminal'
								: 'Open a new shell'
						}
						className={cn(
							'inline-flex shrink-0 cursor-pointer items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-[12px] font-medium transition-colors',
							shells.length >= 3
								? 'cursor-not-allowed text-muted-foreground/40'
								: 'text-muted-foreground hover:bg-foreground/[0.05] hover:text-foreground',
						)}>
						<Plus className="size-3.5" />
						New Shell
					</button>
				</div>

				{/* Shells — all mounted, only one visible (hidden keep streaming) */}
				<div
					className={cn(
						'relative min-h-0 flex-1',
						expanded ? '' : 'h-[60vh] min-h-[480px]',
					)}>
					{shells.length === 0 && (
						<div className="absolute inset-0 flex flex-col items-center justify-center gap-3 text-center">
							<p className="text-sm text-muted-foreground/70">
								No terminals
							</p>
							<button
								type="button"
								onClick={handleAdd}
								className="inline-flex cursor-pointer items-center gap-1.5 rounded-xl border border-border/50 bg-muted/30 px-4 py-2 text-[13px] font-medium text-foreground transition-colors hover:bg-muted/60">
								<Plus className="size-3.5" />
								New Shell
							</button>
						</div>
					)}
					{shells.map(entry => (
						<div
							key={entry.id}
							className={entry.id === activeId ? 'absolute inset-0' : 'hidden'}>
							<ShellSession
								visible={entry.id === activeId}
								enabled={enabled}
								targetContainer={targetContainer}
								shell={shell}
								isRemoteServer={isRemoteServer}
								hostMode={hostMode}
								serverId={serverId}
								onStatus={status => setStatus(entry.id, status)}
							/>
						</div>
					))}
				</div>
			</div>
		</div>
	);
}
