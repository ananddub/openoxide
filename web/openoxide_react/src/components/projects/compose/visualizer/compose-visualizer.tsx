import {useMemo} from 'react';
import {ReactFlowProvider} from '@xyflow/react';
import {cn} from '#/api/utils';
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
	serviceStatus,
	wrapperClassName,
	...rest
}: ComposeVisualizerProps) {
	const services = useMemo(() => {
		let base: ComposeService[];
		if (customServices && customServices.length > 0) {
			base = customServices;
		} else {
			const parsed = parseComposeFile(composeFile ?? '');
			if (parsed.length > 0) {
				base = parsed;
			} else if (stackName && stackName.trim()) {
				const stub: ComposeService = {
					name: stackName.trim(),
					image: 'docker-compose stack',
					dependsOn: [],
					envVars: {},
					volumes: [],
					ports: [],
				};
				base = [stub];
			} else {
				base = [];
			}
		}
		if (!serviceStatus) return base;
		return base.map(svc =>
			svc.status ? svc : {...svc, status: serviceStatus[svc.name]},
		);
	}, [composeFile, customServices, stackName, serviceStatus]);

	return (
		<div
			className={cn(
				'overflow-hidden rounded-[14px] border border-border bg-background',
				wrapperClassName ?? 'h-[calc(100dvh_-_240px)] min-h-[420px]',
			)}>
			<ReactFlowProvider>
				<CanvasInner services={services} {...rest} />
			</ReactFlowProvider>
		</div>
	);
}
