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
	SelectValue,
} from '#/components/ui/select';
import {Badge} from '#/components/ui/badge';
import {
	Tabs,
	TabsList,
	TabsTrigger,
	TabsContent,
} from '#/components/ui/tabs';
import {
	Copy,
	Check,
	LogOut,
	RefreshCw,
	ShieldCheck,
	Zap,
	CheckCircle2,
} from 'lucide-react';
import {useState} from 'react';
import {useQuery} from '@tanstack/react-query';
import {toast} from 'sonner';
import {client} from '#/api/client';

import type {RemoteServerResponse, SwarmTokens} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';

interface JoinTokensModalProps {
	isOpen: boolean;
	tokens: SwarmTokens | null;
	isLoading: boolean;
	servers?: RemoteServerResponse[];
	nodes?: TaggedSwarmNode[];
	onLeaveRemoteSwarm?: (serverId: number) => Promise<void>;
	onJoinServer?: (
		serverId: number,
		role: 'worker' | 'manager',
	) => Promise<void>;
	onClose: () => void;
}

function CommandBlock({
	label,
	command,
	isLoading,
}: {
	label: string;
	command: string;
	isLoading: boolean;
}) {
	const [copied, setCopied] = useState(false);
	const disabled = isLoading || !command;

	const handleCopy = () => {
		if (!command) return;
		navigator.clipboard.writeText(command);
		setCopied(true);
		toast.success(
			`${label} join script copied — paste it on the target server.`,
		);
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="w-full min-w-0 space-y-1.5 rounded-lg border border-border/60 bg-muted/20 p-2.5">
			<div className="flex items-center justify-between">
				<span className="text-xs font-semibold text-foreground">
					{label}
				</span>
				<Button
					type="button"
					variant="ghost"
					size="xs"
					disabled={disabled}
					onClick={handleCopy}
					className="h-6 gap-1 p-1 text-[11px] font-semibold text-primary hover:text-primary">
					{copied ? (
						<Check className="h-3 w-3 text-emerald-500" />
					) : (
						<Copy className="h-3 w-3" />
					)}
					{copied ? 'Copied' : 'Copy'}
				</Button>
			</div>
			<div className="max-h-24 w-full min-w-0 overflow-y-auto rounded border border-zinc-800/80 bg-zinc-950 p-2.5 font-mono text-[11px] leading-relaxed break-all text-zinc-200 select-all">
				{isLoading ? (
					<span className="text-zinc-500">Loading token…</span>
				) : command ? (
					command
				) : (
					<span className="text-zinc-500">Token unavailable</span>
				)}
			</div>
		</div>
	);
}

export function JoinTokensModal({
	isOpen,
	tokens,
	isLoading,
	servers = [],
	nodes = [],
	onLeaveRemoteSwarm,
	onJoinServer,
	onClose,
}: JoinTokensModalProps) {
	const [selectedRemoteServerId, setSelectedRemoteServerId] =
		useState<string>('');
	const [role, setRole] = useState<'worker' | 'manager'>('worker');
	const [isLeaving, setIsLeaving] = useState(false);
	const [isJoining, setIsJoining] = useState(false);

	const selectedRemoteServer = servers.find(
		s => String(s.id) === selectedRemoteServerId,
	);

	// `_serverId` on `nodes` only records which query happened to discover a
	// node, not which registered server it actually is — useless for "is
	// server X part of this cluster". Ask server X directly for its own
	// swarm node_id and check whether that id shows up in this cluster's
	// member list; that's the only reliable identity link we have.
	const selectedIdentityQuery = useQuery({
		queryKey: ['swarm-node-identity', selectedRemoteServerId],
		queryFn: async () => {
			const {data} = await client.POST('/swarm/info', {
				body: {server_id: parseInt(selectedRemoteServerId, 10)},
			});
			return data ?? null;
		},
		enabled: !!selectedRemoteServerId,
		staleTime: 10_000,
	});

	const isCheckingStatus = selectedIdentityQuery.isLoading;
	const isSelectedConnected = !!(
		selectedIdentityQuery.data?.node_id &&
		nodes.some(n => n.id === selectedIdentityQuery.data?.node_id)
	);

	const handleDisconnect = async () => {
		if (!selectedRemoteServerId || !onLeaveRemoteSwarm) return;
		setIsLeaving(true);
		try {
			await onLeaveRemoteSwarm(parseInt(selectedRemoteServerId, 10));
		} catch {
			// handled by parent
		} finally {
			setIsLeaving(false);
		}
	};

	const handleJoin = async () => {
		if (!selectedRemoteServerId || !onJoinServer) return;
		setIsJoining(true);
		try {
			await onJoinServer(parseInt(selectedRemoteServerId, 10), role);
		} catch {
			// handled by parent
		} finally {
			setIsJoining(false);
		}
	};

	const workerCmd = tokens?.worker
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.worker}`
		: '';
	const managerCmd = tokens?.manager
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.manager}`
		: '';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-h-[85vh] w-full overflow-y-auto rounded-xl border-border bg-card p-6 shadow-xl sm:max-w-lg">
				<DialogHeader className="border-b border-border/40 pb-3">
					<DialogTitle className="text-base font-bold text-foreground">
						Add Node to Swarm Cluster
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Connect a server as a Worker or Manager
					</DialogDescription>
				</DialogHeader>

				<Tabs defaultValue="registered" className="w-full pt-1">
					<TabsList className="w-full">
						<TabsTrigger
							value="registered"
							className="text-xs font-semibold">
							Registered Servers
						</TabsTrigger>
						<TabsTrigger value="manual" className="text-xs font-semibold">
							Manual / External
						</TabsTrigger>
					</TabsList>

					{/* ── Registered servers: automated 1-click join ── */}
					<TabsContent
						value="registered"
						className="flex flex-col gap-3 pt-3 focus-visible:outline-none">
						{servers.length === 0 ? (
							<p className="py-6 text-center text-xs text-muted-foreground">
								No registered remote servers yet. Add one first, or use the
								Manual tab for external machines.
							</p>
						) : (
							<>
								<div className="flex flex-col gap-1.5">
									<label className="text-xs font-semibold text-foreground">
										Server
									</label>
									<Select
										value={selectedRemoteServerId}
										onValueChange={val =>
											val && setSelectedRemoteServerId(val)
										}>
										<SelectTrigger className="h-9 w-full border-border bg-background text-xs">
											<SelectValue placeholder="Select a server…">
												{selectedRemoteServer
													? `${selectedRemoteServer.name} (${selectedRemoteServer.ip_address})`
													: 'Select a server…'}
											</SelectValue>
										</SelectTrigger>
										<SelectContent>
											{servers.map(s => (
												<SelectItem key={s.id} value={String(s.id)}>
													{s.name} ({s.ip_address})
												</SelectItem>
											))}
										</SelectContent>
									</Select>
								</div>

								{selectedRemoteServer && (
									<div className="flex flex-col gap-3 rounded-lg border border-border/50 bg-muted/20 p-3">
										<div className="flex items-center gap-2">
											<span className="text-[11px] font-semibold text-muted-foreground">
												Status
											</span>
											{isCheckingStatus ? (
												<Badge
													variant="outline"
													className="gap-1 px-1.5 py-0 text-[10px] text-muted-foreground">
													<RefreshCw className="h-3 w-3 animate-spin" />{' '}
													Checking…
												</Badge>
											) : isSelectedConnected ? (
												<Badge
													variant="secondary"
													className="gap-1 px-1.5 py-0 text-[10px]">
													<CheckCircle2 className="h-3 w-3 text-emerald-500" />{' '}
													Already in this cluster
												</Badge>
											) : (
												<Badge
													variant="outline"
													className="gap-1 px-1.5 py-0 text-[10px] text-muted-foreground">
													Not in this cluster
												</Badge>
											)}
										</div>

										{isCheckingStatus ? null : isSelectedConnected ? (
											<Button
												variant="outline"
												size="sm"
												disabled={isLeaving}
												onClick={handleDisconnect}
												className="h-8 gap-1.5 self-start border-destructive/40 text-xs font-semibold text-destructive hover:bg-destructive/10 hover:text-destructive">
												{isLeaving ? (
													<RefreshCw className="h-3.5 w-3.5 animate-spin" />
												) : (
													<LogOut className="h-3.5 w-3.5" />
												)}
												{isLeaving
													? 'Disconnecting…'
													: 'Disconnect from Cluster'}
											</Button>
										) : (
											<div className="flex items-end gap-2">
												<div className="flex w-32 flex-col gap-1.5">
													<label className="text-[11px] font-semibold text-foreground">
														Role
													</label>
													<Select
														value={role}
														onValueChange={v =>
															setRole(v as 'worker' | 'manager')
														}>
														<SelectTrigger
															size="sm"
															className="h-8 w-full border-border bg-background text-xs">
															<SelectValue />
														</SelectTrigger>
														<SelectContent>
															<SelectItem value="worker">
																Worker
															</SelectItem>
															<SelectItem value="manager">
																Manager
															</SelectItem>
														</SelectContent>
													</Select>
												</div>
												<Button
													size="sm"
													disabled={isJoining}
													onClick={handleJoin}
													className="h-8 flex-1 gap-1.5 text-xs font-semibold">
													{isJoining ? (
														<RefreshCw className="h-3.5 w-3.5 animate-spin" />
													) : (
														<Zap className="h-3.5 w-3.5" />
													)}
													{isJoining ? 'Joining…' : 'Join Cluster'}
												</Button>
											</div>
										)}
									</div>
								)}
							</>
						)}
					</TabsContent>

					{/* ── Manual scripts, for machines not managed by OpenOxide ── */}
					<TabsContent
						value="manual"
						className="flex flex-col gap-3 pt-3 focus-visible:outline-none">
						<p className="text-[11px] text-muted-foreground">
							Paste one of these on an external machine over SSH to join it
							into this cluster.
						</p>
						<CommandBlock
							label="Worker join script"
							command={workerCmd}
							isLoading={isLoading}
						/>
						<CommandBlock
							label="Manager join script"
							command={managerCmd}
							isLoading={isLoading}
						/>
						<p className="flex items-center gap-1 text-[10px] text-muted-foreground">
							<ShieldCheck className="h-3 w-3 shrink-0" />
							Managers get full cluster control — only share this script
							with trusted machines.
						</p>
					</TabsContent>
				</Tabs>
			</DialogContent>
		</Dialog>
	);
}
