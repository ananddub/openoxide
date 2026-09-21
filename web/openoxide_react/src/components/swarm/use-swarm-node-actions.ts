import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';

type Mutation = {mutateAsync: (args: any) => Promise<unknown>};

interface NodeActionArgs {
	promoteMutation: Mutation;
	demoteMutation: Mutation;
	availabilityMutation: Mutation;
	removeNodeMutation: Mutation;
	getServerIdNumber: (node?: TaggedSwarmNode) => number | undefined;
	refreshAll: () => void;
}

/** Promote / demote / availability / remove handlers for swarm nodes. */
export function useSwarmNodeActions({
	promoteMutation,
	demoteMutation,
	availabilityMutation,
	removeNodeMutation,
	getServerIdNumber,
	refreshAll,
}: NodeActionArgs) {
	const handlePromote = async (nodeId: string, node?: TaggedSwarmNode) => {
		try {
			await promoteMutation.mutateAsync({
				server_id: getServerIdNumber(node),
				node_id: nodeId,
			});
			toast.success('Node promoted to Manager');
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDemote = async (nodeId: string, node?: TaggedSwarmNode) => {
		try {
			await demoteMutation.mutateAsync({
				server_id: getServerIdNumber(node),
				node_id: nodeId,
			});
			toast.success('Node demoted to Worker');
			refreshAll();
		} catch (err: unknown) {
			const rawErr = formatApiError(err);
			if (rawErr.toLowerCase().includes('last manager')) {
				toast.error(
					'Promote another node to Manager first — a swarm always needs at least one.',
				);
			} else {
				toast.error(rawErr);
			}
		}
	};

	const handleSetAvailability = async (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => {
		try {
			await availabilityMutation.mutateAsync({
				server_id: getServerIdNumber(node),
				node_id: nodeId,
				availability,
			});
			toast.success(`Node availability set to ${availability}`);
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleRemoveNode = async (
		nodeId: string,
		node?: TaggedSwarmNode,
	) => {
		try {
			await removeNodeMutation.mutateAsync({
				server_id: getServerIdNumber(node),
				node_id: nodeId,
			});
			toast.success('Node removed from Swarm cluster');
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	return {
		handlePromote,
		handleDemote,
		handleSetAvailability,
		handleRemoveNode,
	};
}
