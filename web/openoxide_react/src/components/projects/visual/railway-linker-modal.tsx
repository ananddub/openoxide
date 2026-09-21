import {ServiceLinkerModal} from './modals/service-linker-modal';
import type {PendingConn} from './use-service-linker';

/** App↔DB linker modal driven by a pending connection. */
export function RailwayLinkerModal({
	pending,
	onClose,
}: {
	pending: PendingConn;
	onClose: () => void;
}) {
	return (
		<ServiceLinkerModal
			isOpen
			onClose={onClose}
			sourceNode={{
				id: pending.sourceId,
				name: pending.sourceName,
				type: 'app',
			}}
			targetNode={{
				id: pending.targetId,
				name: pending.targetName,
				type: 'database',
				dbType: pending.targetDbType,
			}}
			onConfirmLink={async () => onClose()}
		/>
	);
}
