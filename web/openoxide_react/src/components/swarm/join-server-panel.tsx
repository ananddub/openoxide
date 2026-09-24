import {useState} from 'react';
import {useQuery} from '@tanstack/react-query';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Button} from '#/components/ui/button';
import {WizardField} from '#/components/shared/sheet-wizard';
import {LogOut, RefreshCw, Server, Zap} from 'lucide-react';
import {swarmInfo} from '#/api/live';
import {JoinStatusBadge} from '#/components/swarm/join-status-badge';
import type {RemoteServerResponse} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';

interface JoinServerPanelProps {
	servers: RemoteServerResponse[];
	nodes: TaggedSwarmNode[];
	onLeaveRemoteSwarm?: (serverId: number) => Promise<void>;
	onJoinServer?: (
		serverId: number,
		role: 'worker' | 'manager',
	) => Promise<void>;
}

/** Registered-servers mode: pick an engine, then join or disconnect it in one click. */
export function JoinServerPanel({
	servers,
	nodes,
	onLeaveRemoteSwarm,
	onJoinServer,
}: JoinServerPanelProps) {
	const [selectedRemoteServerId, setSelectedRemoteServerId] =
		useState<string>('');
	const [role, setRole] = useState<'worker' | 'manager'>('worker');
	const [isLeaving, setIsLeaving] = useState(false);
	const [isJoining, setIsJoining] = useState(false);

	const selectedRemoteServer = servers.find(
		s => String(s.id) === selectedRemoteServerId,
	);

	// `_serverId` on `nodes` only records which query happened to discover a node, not which registered server it actually is — useless for "is server X part of this cluster". Ask server X directly for its own swarm node_id and check whether that id shows up in this cluster's member list; that's the only reliable identity link we have.
	const selectedIdentityQuery = useQuery({
		queryKey: ['swarm-node-identity', selectedRemoteServerId],
		queryFn: async () => {
			const data = await swarmInfo(parseInt(selectedRemoteServerId, 10));
			return (data as {node_id?: string} | null) ?? null;
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

	if (servers.length === 0) {
		return (
			<div className="flex flex-col items-center gap-1.5 rounded-lg border border-dashed border-border p-6 text-center">
				<Server className="size-5 text-muted-foreground/40" />
				<p className="text-[13px] font-medium text-foreground">
					No registered servers yet
				</p>
				<p className="max-w-xs text-xs text-muted-foreground">
					Register an engine first, or paste the join script from the
					Manual mode onto an external machine.
				</p>
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-5">
			<WizardField
				label="Engine"
				hint="Servers you registered in OpenOxide — the panel talks to each one over SSH.">
				<Select
					value={selectedRemoteServerId}
					onValueChange={val =>
						val && setSelectedRemoteServerId(val)
					}>
					<SelectTrigger>
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
			</WizardField>

			{selectedRemoteServer && (
				<div className="flex flex-col gap-4 rounded-xl border border-border/50 p-4">
					<div className="flex items-center gap-3">
						<span className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted/30 text-muted-foreground">
							<Server className="size-4" />
						</span>
						<span className="min-w-0 flex-1">
							<span className="block truncate text-sm font-medium text-foreground">
								{selectedRemoteServer.name}
							</span>
							<span className="block truncate font-mono text-xs text-muted-foreground">
								{selectedRemoteServer.ip_address}
							</span>
						</span>
						<JoinStatusBadge
							isChecking={isCheckingStatus}
							isConnected={isSelectedConnected}
						/>
					</div>

					{isCheckingStatus ? null : isSelectedConnected ? (
						<Button
							variant="outline"
							disabled={isLeaving}
							onClick={handleDisconnect}
							className="w-full border-destructive/40 text-destructive hover:bg-destructive/10 hover:text-destructive">
							{isLeaving ? (
								<RefreshCw className="size-4 animate-spin" />
							) : (
								<LogOut className="size-4" />
							)}
							{isLeaving
								? 'Disconnecting…'
								: 'Disconnect from Cluster'}
						</Button>
					) : (
						<div className="flex items-end gap-2.5">
							<div className="w-36 shrink-0">
								<WizardField label="Role">
									<Select
										value={role}
										onValueChange={v =>
											setRole(v as 'worker' | 'manager')
										}>
										<SelectTrigger>
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
								</WizardField>
							</div>
							<Button
								disabled={isJoining}
								onClick={handleJoin}
								className="flex-1">
								{isJoining ? (
									<RefreshCw className="size-4 animate-spin" />
								) : (
									<Zap className="size-4" />
								)}
								{isJoining ? 'Joining…' : 'Join Cluster'}
							</Button>
						</div>
					)}
				</div>
			)}
		</div>
	);
}
