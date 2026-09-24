import {ListRow} from '#/components/settings/list-card';
import {
	SwarmNodeActions,
	SwarmNodeMeta,
	SwarmNodeTile,
	SwarmRoleBadge,
	SwarmStatusBadge,
} from './swarm-node-parts';
import type {TaggedSwarmNode} from './swarm-node-shared';

interface SwarmNodeRowProps {
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

/** List row — the `ListRow` counterpart of `SwarmNodeCard`. */
export function SwarmNodeRow(props: SwarmNodeRowProps) {
	const {node} = props;
	const availability = (node.availability || 'active').toLowerCase();

	return (
		<ListRow
			icon={<SwarmNodeTile node={node} />}
			title={node.hostname || 'Swarm Node'}
			badges={
				<>
					<SwarmRoleBadge node={node} />
					<SwarmStatusBadge node={node} />
				</>
			}
			subtitle={<SwarmNodeMeta node={node} availability={availability} />}>
			<SwarmNodeActions {...props} />
		</ListRow>
	);
}
