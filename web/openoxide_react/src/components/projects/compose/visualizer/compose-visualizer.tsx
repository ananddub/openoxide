import {useMemo} from 'react';
import {ReactFlowProvider} from '@xyflow/react';
import {parseComposeFile, type ComposeService} from './compose-parse';
import {CanvasInner} from './compose-canvas';
import type {ComposeVisualizerProps} from './compose-canvas-types';

export type {ComposeService} from './compose-parse';
export {parseComposeFile} from './compose-parse';
export type {ComposeVisualizerProps} from './compose-canvas-types';

// ─── Public export ────────────────────────────────────────────────────────────

export function ComposeVisualizer({
	composeFile,
	stackName,
	customServices,
	...rest
}: ComposeVisualizerProps) {
	const services = useMemo(() => {
		if (customServices && customServices.length > 0) return customServices;
		const parsed = parseComposeFile(composeFile ?? '');
		if (parsed.length > 0) return parsed;

		if (stackName && stackName.trim()) {
			const stub: ComposeService = {
				name: stackName.trim(),
				image: 'docker-compose stack',
				dependsOn: [],
				envVars: {},
				volumes: [],
				ports: [],
			};
			return [stub];
		}
		return [];
	}, [composeFile, customServices, stackName]);

	return (
		<div
			style={{
				height: 'calc(100vh - 320px)',
				minHeight: 410,
				borderRadius: 14,
				overflow: 'hidden',
				border: '1px solid rgba(255,255,255,0.08)',
				background: '#0d1117',
			}}>
			<ReactFlowProvider>
				<CanvasInner services={services} {...rest} />
			</ReactFlowProvider>
		</div>
	);
}
