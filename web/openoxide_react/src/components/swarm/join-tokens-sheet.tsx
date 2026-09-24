import {useState} from 'react';
import {
	Sheet,
	SheetClose,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Server, ShieldCheck, Terminal} from 'lucide-react';

import type {RemoteServerResponse, SwarmTokens} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';
import {
	OptionRow,
	StepBody,
} from '#/components/shared/sheet-wizard';
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

type JoinMode = 'registered' | 'manual';

const METHODS: {
	key: JoinMode;
	title: string;
	subtitle: string;
	icon: typeof Server;
}[] = [
	{
		key: 'registered',
		title: 'Registered engine',
		subtitle: 'Attach a server OpenOxide already reaches over SSH.',
		icon: Server,
	},
	{
		key: 'manual',
		title: 'External machine',
		subtitle: 'Run the join token yourself, over SSH or the console.',
		icon: Terminal,
	},
];

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
	const [mode, setMode] = useState<JoinMode>('registered');
	const workerCmd = tokens?.worker
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.worker}`
		: '';
	const managerCmd = tokens?.manager
		? `docker swarm leave --force 2>/dev/null; docker swarm join --token ${tokens.manager}`
		: '';

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent side="right">
				<SheetHeader>
					<SheetTitle>New Swarm Node</SheetTitle>
					<SheetDescription>
						Attach another Docker engine to this cluster as a worker
						or a manager.
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
					<div className="flex flex-col gap-2">
						{METHODS.map(m => {
							const Icon = m.icon;
							return (
								<OptionRow
									key={m.key}
									selected={mode === m.key}
									onClick={() => setMode(m.key)}
									icon={<Icon className="size-5" />}
									title={m.title}
									subtitle={m.subtitle}
								/>
							);
						})}
					</div>

					<div className="mt-6">
						<StepBody key={mode}>
							{mode === 'registered' ? (
								<JoinServerPanel
									servers={servers}
									nodes={nodes}
									onLeaveRemoteSwarm={onLeaveRemoteSwarm}
									onJoinServer={onJoinServer}
								/>
							) : (
								<div className="flex flex-col gap-3">
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
									<p className="flex items-start gap-2 rounded-lg border border-border/60 bg-muted/20 p-3 text-[13px] text-muted-foreground">
										<ShieldCheck className="mt-0.5 size-4 shrink-0 text-emerald-500" />
										Managers get full cluster control — only
										join machines you trust as managers.
									</p>
								</div>
							)}
						</StepBody>
					</div>
				</div>

				<SheetFooter>
					<SheetClose render={<Button variant="outline" />}>
						Done
					</SheetClose>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
