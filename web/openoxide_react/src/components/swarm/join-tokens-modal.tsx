import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	Tabs,
	TabsList,
	TabsTrigger,
	TabsContent,
} from '#/components/ui/tabs';
import {ShieldCheck} from 'lucide-react';

import type {RemoteServerResponse, SwarmTokens} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';
import {CommandBlock} from '#/components/swarm/join-tokens-command-block';
import {JoinServerPanel} from '#/components/swarm/join-server-panel';

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
	const workerCmd = tokens?.worker
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.worker}`
		: '';
	const managerCmd = tokens?.manager
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.manager}`
		: '';

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="border- max-h-[85vh] w-full overflow-y-auto rounded-xl bg-card p-6 shadow-md sm:max-w-lg">
				<DialogHeader className="border-b  pb-3">
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
						<JoinServerPanel
							servers={servers}
							nodes={nodes}
							onLeaveRemoteSwarm={onLeaveRemoteSwarm}
							onJoinServer={onJoinServer}
						/>
					</TabsContent>

					{/* ── Manual scripts, for machines not managed by OpenOxide ── */}
					<TabsContent
						value="manual"
						className="flex flex-col gap-3 pt-3 focus-visible:outline-none">
						<p className="text-[11px] text-muted-foreground">
							Paste one of these on an external machine over SSH to join
							it into this cluster.
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
							Managers get full cluster control — only share this
							script with trusted machines.
						</p>
					</TabsContent>
				</Tabs>
			</DialogContent>
		</Dialog>
	);
}
