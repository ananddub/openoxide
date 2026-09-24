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
import {LogOut, RefreshCw, Zap} from 'lucide-react';
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

/** Registered-servers tab: pick a server, 1-click join / disconnect. */
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
			<p className="py-6 text-center text-xs text-muted-foreground">
				No registered remote servers yet. Add one first, or use the Manual
				tab for external machines.
			</p>
		);
	}

	return (
		<>
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Server
				</label>
				<Select
					value={selectedRemoteServerId}
					onValueChange={val => val && setSelectedRemoteServerId(val)}>
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
				<div className="flex flex-col gap-3 rounded-lg bg-muted/20 p-3">
					<div className="flex items-center gap-2">
						<span className="text-[11px] font-semibold text-muted-foreground">
							Status
						</span>
						<JoinStatusBadge
							isChecking={isCheckingStatus}
							isConnected={isSelectedConnected}
						/>
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
							{isLeaving ? 'Disconnecting…' : 'Disconnect from Cluster'}
						</Button>
					) : (
						<div className="flex items-end gap-2">
							<div className="flex w-32 flex-col gap-1.5">
								<label className="text-[11px] font-semibold text-foreground">
									Role
								</label>
								<Select
									value={role}
									onValueChange={v => setRole(v as 'worker' | 'manager')}>
									<SelectTrigger
										size="sm"
										className="h-8 w-full border-border bg-background text-xs">
										<SelectValue />
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="worker">Worker</SelectItem>
										<SelectItem value="manager">Manager</SelectItem>
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
	);
}
