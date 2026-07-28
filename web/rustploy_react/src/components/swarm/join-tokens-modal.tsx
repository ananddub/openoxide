import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '#/components/ui/select';
import {Badge} from '#/components/ui/badge';
import {Copy, Check, LogOut, RefreshCw, Server, ShieldCheck, Terminal, Crown, ArrowRight, Zap, CheckCircle2, AlertTriangle} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';

interface JoinTokensModalProps {
	isOpen: boolean;
	tokens: {worker?: string; manager?: string} | null;
	isLoading: boolean;
	servers?: any[];
	nodes?: any[];
	onLeaveRemoteSwarm?: (serverId: number) => Promise<void>;
	onClose: () => void;
}

function HighlightedJoinCommand({command}: {command: string}) {
	if (!command || command.startsWith('Loading') || command === 'N/A') {
		return <span className="text-muted-foreground">{command}</span>;
	}

	const tokenPrefix = 'docker swarm leave --force 2>/dev/null; docker swarm join --token ';
	const rawTokenPrefix = 'docker swarm join --token ';

	let tokenValue = command;
	if (command.startsWith(tokenPrefix)) {
		tokenValue = command.slice(tokenPrefix.length);
	} else if (command.startsWith(rawTokenPrefix)) {
		tokenValue = command.slice(rawTokenPrefix.length);
	} else {
		return <span className="font-mono text-[11px] text-foreground">{command}</span>;
	}

	return (
		<span className="font-mono text-[11px] leading-relaxed">
			<span className="text-amber-500 font-bold">docker swarm leave --force 2&gt;/dev/null;</span>{' '}
			<span className="text-emerald-500 font-bold">docker swarm join</span>{' '}
			<span className="text-amber-500 font-medium">--token</span>{' '}
			<span className="text-foreground font-medium break-all">{tokenValue}</span>
		</span>
	);
}

export function JoinTokensModal({
	isOpen,
	tokens,
	isLoading,
	servers = [],
	nodes = [],
	onLeaveRemoteSwarm,
	onClose,
}: JoinTokensModalProps) {
	const [copiedKey, setCopiedKey] = useState<string | null>(null);
	const [selectedRemoteServerId, setSelectedRemoteServerId] = useState<string>('');
	const [isLeaving, setIsLeaving] = useState(false);

	const selectedRemoteServer = servers.find((s: any) => String(s.id) === selectedRemoteServerId);
	const selectedRemoteLabel = selectedRemoteServer
		? `${selectedRemoteServer.name} (${selectedRemoteServer.ip_address})`
		: 'Select Remote Server';

	const isSelectedConnected = selectedRemoteServer
		? nodes.some((n: any) =>
				(n._serverId !== undefined && String(n._serverId) === selectedRemoteServerId) ||
				(selectedRemoteServer.ip_address && (n.ip === selectedRemoteServer.ip_address || n.Addr === selectedRemoteServer.ip_address))
		  )
		: false;

	const handleCopy = (type: 'worker' | 'manager', token?: string) => {
		if (!token) {
			toast.error('Token not available');
			return;
		}
		const cmd = `docker swarm leave --force 2>/dev/null; docker swarm join --token ${token}`;
		navigator.clipboard.writeText(cmd);
		setCopiedKey(type);
		toast.success(`Copied 1-Step Join Script for ${type}! Paste on target server.`);
		setTimeout(() => setCopiedKey(null), 2000);
	};

	const handleResetServerSwarm = async () => {
		if (!selectedRemoteServerId) {
			toast.error('Select a remote server first');
			return;
		}
		if (!onLeaveRemoteSwarm) return;

		const srvId = parseInt(selectedRemoteServerId);
		setIsLeaving(true);
		try {
			await onLeaveRemoteSwarm(srvId);
		} catch {
			// handled gracefully in parent page
		} finally {
			setIsLeaving(false);
		}
	};

	const workerCmd = isLoading
		? 'Loading worker token...'
		: tokens?.worker
			? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.worker}`
			: 'N/A';

	const managerCmd = isLoading
		? 'Loading manager token...'
		: tokens?.manager
			? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.manager}`
			: 'N/A';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl w-full bg-card border-border p-6 shadow-2xl rounded-2xl max-h-[90vh] overflow-y-auto">
				<DialogHeader className="pb-3 border-b border-border/50">
					<DialogTitle className="text-base font-bold text-foreground">
						Add Node to Swarm Cluster
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Connect new servers (Workers or Managers) to your Main Swarm Leader Cluster
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-5 mt-2 text-xs">
					{/* Target Leader Cluster Info Box */}
					<div className="bg-primary/5 border border-primary/20 p-3 rounded-xl flex items-center justify-between gap-3">
						<div className="flex items-center gap-2.5 min-w-0">
							<div className="p-1.5 rounded-lg bg-amber-500/10 text-amber-500 shrink-0">
								<Crown className="size-4" />
							</div>
							<div className="flex flex-col min-w-0">
								<span className="text-[10px] font-mono font-bold text-muted-foreground uppercase tracking-wider">Target Leader Cluster</span>
								<span className="font-bold text-foreground truncate">Primary Swarm Manager (Local Engine)</span>
							</div>
						</div>

						<div className="flex items-center gap-1 text-primary shrink-0 font-mono text-[10px] font-bold bg-primary/10 px-2.5 py-1 rounded-md border border-primary/20">
							<span>Active Target</span>
							<ArrowRight className="size-3" />
						</div>
					</div>

					{/* Section 1: Worker Join Command */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<div className="flex items-center gap-1.5 font-semibold text-foreground">
								<Terminal className="size-3.5 text-emerald-500" />
								<span>Worker 1-Step Join Script</span>
								<Badge variant="secondary" className="text-[9px] uppercase px-1.5 py-0 font-mono">
									Auto Reset + Join
								</Badge>
							</div>
							<Button
								variant="outline"
								size="sm"
								onClick={() => handleCopy('worker', tokens?.worker)}
								className="h-7 text-xs font-bold gap-1 px-2.5 cursor-pointer shadow-2xs bg-emerald-500/10 text-emerald-500 border-emerald-500/30 hover:bg-emerald-500/20"
							>
								{copiedKey === 'worker' ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedKey === 'worker' ? 'Copied Script' : 'Copy 1-Step Script'}
							</Button>
						</div>
						<div className="bg-zinc-950 dark:bg-zinc-950 p-3 rounded-xl border border-zinc-800/80 shadow-inner overflow-x-auto select-all">
							<HighlightedJoinCommand command={workerCmd} />
						</div>
					</div>

					{/* Section 2: Manager Join Command */}
					<div className="flex flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<div className="flex items-center gap-1.5 font-semibold text-foreground">
								<ShieldCheck className="size-3.5 text-amber-500" />
								<span>Manager 1-Step Join Script</span>
								<Badge variant="outline" className="text-[9px] uppercase px-1.5 py-0 font-mono text-amber-500 border-amber-500/30">
									High Availability
								</Badge>
							</div>
							<Button
								variant="outline"
								size="sm"
								onClick={() => handleCopy('manager', tokens?.manager)}
								className="h-7 text-xs font-bold gap-1 px-2.5 cursor-pointer shadow-2xs bg-amber-500/10 text-amber-500 border-amber-500/30 hover:bg-amber-500/20"
							>
								{copiedKey === 'manager' ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
								{copiedKey === 'manager' ? 'Copied Script' : 'Copy 1-Step Script'}
							</Button>
						</div>
						<div className="bg-zinc-950 dark:bg-zinc-950 p-3 rounded-xl border border-zinc-800/80 shadow-inner overflow-x-auto select-all">
							<HighlightedJoinCommand command={managerCmd} />
						</div>
					</div>

					{/* Section 3: Smart Auto-Detected Remote Server Actions */}
					{servers.length > 0 && onLeaveRemoteSwarm && (
						<div className="flex flex-col gap-2.5 pt-4 border-t border-border/50">
							<div className="flex items-center justify-between">
								<span className="font-semibold text-foreground flex items-center gap-1.5">
									<Server className="size-3.5 text-primary" />
									1-Click Remote Swarm Disconnect & Auto-Reset
								</span>
							</div>
							<p className="text-[11px] text-muted-foreground leading-normal">
								Select a remote server below. Click the button to automatically force-disconnect its standalone Swarm over SSH/API.
							</p>

							<div className="flex flex-col gap-2 mt-1">
								<Select value={selectedRemoteServerId} onValueChange={val => val && setSelectedRemoteServerId(val)}>
									<SelectTrigger className="h-9 text-xs bg-card border-border/80 w-full px-3 flex items-center justify-between">
										<span className="truncate font-semibold text-foreground">{selectedRemoteLabel}</span>
									</SelectTrigger>
									<SelectContent className="bg-card border-border text-xs z-50">
										{servers.map((s: any) => {
											const isConnected = nodes.some((n: any) =>
												(n._serverId !== undefined && String(n._serverId) === String(s.id)) ||
												(s.ip_address && (n.ip === s.ip_address || n.Addr === s.ip_address))
											);
											return (
												<SelectItem key={s.id} value={String(s.id)}>
													{s.name} ({s.ip_address}) {isConnected ? '✓ Connected' : '• Ready to Join'}
												</SelectItem>
											);
										})}
									</SelectContent>
								</Select>

								{/* Smart Auto-Detected Action Buttons */}
								{selectedRemoteServer && (
									<div className="flex items-center gap-2 flex-wrap mt-1 p-3 bg-muted/40 border border-border/60 rounded-xl">
										<div className="flex items-center gap-2 w-full mb-1">
											<span className="text-[11px] font-semibold text-foreground">Status:</span>
											{isSelectedConnected ? (
												<Badge className="bg-emerald-500/10 text-emerald-500 border border-emerald-500/30 text-[10px] gap-1 px-2">
													<CheckCircle2 className="size-3" /> Connected to Main Cluster
												</Badge>
											) : (
												<Badge variant="outline" className="text-amber-500 border-amber-500/30 bg-amber-500/10 text-[10px] gap-1 px-2">
													<AlertTriangle className="size-3" /> Standalone Swarm / Ready to Join
												</Badge>
											)}
										</div>

										<div className="flex items-center gap-2 flex-wrap w-full">
											<Button
												variant="outline"
												size="sm"
												disabled={isLeaving}
												onClick={handleResetServerSwarm}
												className="h-8 text-xs font-bold border-destructive/50 text-destructive hover:bg-destructive/10 gap-1.5 px-3 cursor-pointer shadow-2xs"
											>
												{isLeaving ? <RefreshCw className="size-3.5 animate-spin" /> : <LogOut className="size-3.5" />}
												{isLeaving ? 'Auto-Disconnecting...' : '1-Click Auto-Disconnect Swarm'}
											</Button>

											{!isSelectedConnected && (
												<Button
													variant="default"
													size="sm"
													onClick={() => {
														handleCopy('worker', tokens?.worker);
													}}
													className="h-8 text-xs font-bold bg-emerald-600 hover:bg-emerald-700 text-white gap-1.5 px-3 ml-auto cursor-pointer shadow-2xs"
												>
													<Zap className="size-3.5" />
													<span>Copy 1-Step Join Script</span>
												</Button>
											)}
										</div>
									</div>
								)}
							</div>
						</div>
					)}
				</div>
			</DialogContent>
		</Dialog>
	);
}
