import {Handle, Position} from '@xyflow/react';
import {type ComposeService} from './compose-visualizer';

export type ComposeServiceNodeData = ComposeService & {
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
	readOnly?: boolean;
	/** Live docker-stats target for this node (single-service stacks only). */
	monitorId?: number;
	monitorType?: 'application' | 'database' | 'compose';
};

export {DockerServiceIcon as ServiceIcon} from '../../common/docker-service-icon';

// ─── React Flow connection handles ────────────────────────────────────────────

const handleStyle = {
	width: 8,
	height: 8,
	background: 'var(--muted-foreground)',
	border: '2px solid var(--card)',
} as const;

export function NodeHandles() {
	return (
		<>
			<Handle
				type="target"
				position={Position.Left}
				style={{...handleStyle, left: -5}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{...handleStyle, right: -5}}
			/>
		</>
	);
}
