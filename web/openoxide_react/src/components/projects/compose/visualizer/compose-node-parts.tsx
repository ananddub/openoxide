import {Handle, Position} from '@xyflow/react';
import {type ComposeService} from './compose-visualizer';

export type ComposeServiceNodeData = ComposeService & {
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
};

export {DockerServiceIcon as ServiceIcon} from '../../common/docker-service-icon';

// ─── React Flow connection handles ────────────────────────────────────────────

const handleStyle = {
	width: 8,
	height: 8,
	background: '#30363d',
	border: '2px solid #0d1117',
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
