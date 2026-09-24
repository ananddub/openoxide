import {ViewCard} from '#/components/settings/list-card';
import {
	SwarmNodeActions,
	SwarmNodeMeta,
	SwarmNodeTile,
	SwarmRoleBadge,
	SwarmStatusBadge,
} from './swarm-node-parts';
import type {TaggedSwarmNode} from './swarm-node-shared';

interface SwarmNodeCardProps {
	node: TaggedSwarmNode;
	isLastManager: boolean;
	copiedId: string | null;
	onCopyId: (id: string) => void;
	onPromote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onDemote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onSetAvailability: (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => void;
	onRemoveNode: (nodeId: string, node?: TaggedSwarmNode) => void;
}

/** Grid card — the `ViewCard` counterpart of `SwarmNodeRow`. */
export function SwarmNodeCard(props: SwarmNodeCardProps) {
	const {node} = props;
	const availability = (node.availability || 'active').toLowerCase();

	return (
		<ViewCard>
			<div className="flex min-w-0 items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<SwarmNodeTile node={node} />
					<div className="min-w-0 flex-1">
						<span className="block truncate text-sm font-medium text-foreground">
							{node.hostname || 'Swarm Node'}
						</span>
						<p className="mt-0.5 truncate text-xs text-muted-foreground">
							<SwarmNodeMeta
								node={node}
								availability={availability}
							/>
						</p>
					</div>
				</div>

				<div className="flex shrink-0 items-center gap-1.5 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
					<SwarmNodeActions {...props} />
				</div>
			</div>

			<div className="flex flex-wrap items-center gap-1.5">
				<SwarmRoleBadge node={node} />
				<SwarmStatusBadge node={node} />
			</div>
		</ViewCard>
	);
}
