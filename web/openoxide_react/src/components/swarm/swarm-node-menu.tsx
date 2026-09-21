import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	ArrowUpRight,
	ArrowDownRight,
	ShieldCheck,
	MoreVertical,
	Trash2,
} from 'lucide-react';
import type {TaggedSwarmNode} from './swarm-node-shared';

interface SwarmNodeMenuProps {
	node: TaggedSwarmNode;
	nodeId: string;
	isManager: boolean;
	isLastManager: boolean;
	availability: string;
	onPromote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onDemote: (nodeId: string, node?: TaggedSwarmNode) => void;
	onSetAvailability: (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => void;
	onRemoveNode: (nodeId: string, node?: TaggedSwarmNode) => void;
}

/** The 3-dot action menu for a swarm node row. */
export function SwarmNodeMenu({
	node,
	nodeId,
	isManager,
	isLastManager,
	availability,
	onPromote,
	onDemote,
	onSetAvailability,
	onRemoveNode,
}: SwarmNodeMenuProps) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<button className="flex size-6 shrink-0 cursor-pointer items-center justify-center text-muted-foreground transition-colors hover:text-foreground" />
				}>
				<MoreVertical className="size-4" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" className="w-44">
				{isManager ? (
					<DropdownMenuItem
						disabled={isLastManager}
						title={
							isLastManager
								? "Can't demote the last manager of a swarm"
								: undefined
						}
						className="cursor-pointer gap-2"
						onClick={() => onDemote(nodeId, node)}>
						<ArrowDownRight className="h-3.5 w-3.5" />
						{isLastManager
							? 'Demote to Worker (last manager)'
							: 'Demote to Worker'}
					</DropdownMenuItem>
				) : (
					<DropdownMenuItem
						className="cursor-pointer gap-2"
						onClick={() => onPromote(nodeId, node)}>
						<ArrowUpRight className="h-3.5 w-3.5" /> Promote to Manager
					</DropdownMenuItem>
				)}
				<DropdownMenuSeparator />
				<DropdownMenuItem
					disabled={availability === 'active'}
					className="cursor-pointer gap-2"
					onClick={() => onSetAvailability(nodeId, 'active', node)}>
					<ShieldCheck className="h-3.5 w-3.5" /> Set Active
				</DropdownMenuItem>
				<DropdownMenuItem
					disabled={availability === 'pause'}
					className="cursor-pointer gap-2"
					onClick={() => onSetAvailability(nodeId, 'pause', node)}>
					Set Pause
				</DropdownMenuItem>
				<DropdownMenuItem
					disabled={availability === 'drain'}
					className="cursor-pointer gap-2"
					onClick={() => onSetAvailability(nodeId, 'drain', node)}>
					Set Drain
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="cursor-pointer gap-2 text-destructive focus:text-destructive"
					onClick={() => onRemoveNode(nodeId, node)}>
					<Trash2 className="h-3.5 w-3.5" /> Remove Node
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
