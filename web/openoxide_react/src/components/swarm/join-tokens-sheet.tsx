import {useState} from 'react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {GeistSwitch} from '#/components/shared/geist-switch';
import {ShieldCheck} from 'lucide-react';

import type {RemoteServerResponse, SwarmTokens} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';
import {CommandBlock} from '#/components/swarm/join-tokens-command-block';
import {JoinServerPanel} from '#/components/swarm/join-server-panel';

interface JoinTokensSheetProps {
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

/** New Swarm Node — right sheet, same grammar as the other create surfaces. */
export function JoinTokensSheet({
	isOpen,
	tokens,
	isLoading,
	servers = [],
	nodes = [],
	onLeaveRemoteSwarm,
	onJoinServer,
	onClose,
}: JoinTokensSheetProps) {
	const [mode, setMode] = useState<'registered' | 'manual'>('registered');
	const workerCmd = tokens?.worker
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.worker}`
		: '';
	const managerCmd = tokens?.manager
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.manager}`
		: '';

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>New Swarm Node</SheetTitle>
					<SheetDescription>
						Connect a server to this cluster as a Worker or Manager.
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto px-6 py-6">
					<GeistSwitch
						fill
						options={[
							{key: 'registered', label: 'Registered Servers'},
							{key: 'manual', label: 'Manual / External'},
						]}
						value={mode}
						onChange={setMode}
					/>

					{mode === 'registered' && (
						<JoinServerPanel
							servers={servers}
							nodes={nodes}
							onLeaveRemoteSwarm={onLeaveRemoteSwarm}
							onJoinServer={onJoinServer}
						/>
					)}

					{mode === 'manual' && (
						<div className="flex flex-col gap-3">
							<p className="text-[11px] text-muted-foreground">
								Paste one of these on an external machine over SSH to
								join it into this cluster.
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
								Managers get full cluster control — only join trusted
								machines as Managers.
							</p>
						</div>
					)}
				</div>
			</SheetContent>
		</Sheet>
	);
}
