import {useCallback, useState} from 'react';
import {
	ReactFlow,
	ReactFlowProvider,
	Background,
	BackgroundVariant,
	Controls,
	type Node,
	type NodeTypes,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {
	StackServiceCard,
	type StackServiceData,
} from './stack-service-card';
import {StackConfigPanel} from './stack-config-panel';
import {StackEdge} from './stack-edge';
import {AddServiceModal} from './add-service-modal';
import {StackAddButton, StackEmptyState} from './stack-canvas-overlays';
import {useStackCanvasGraph} from './use-stack-canvas-graph';

// ─── Canvas ───────────────────────────────────────────────────────────────────

const nodeTypes: NodeTypes = {stackService: StackServiceCard as any};
const edgeTypes = {stackEdge: StackEdge};

function CanvasInner({environmentId}: {environmentId: number}) {
	const {
		nodes,
		edges,
		onNodesChange,
		onEdgesChange,
		addService,
		updateNode: graphUpdate,
		deleteNode: graphDelete,
	} = useStackCanvasGraph();
	const [showAdd, setShowAdd] = useState(false);
	const [selected, setSelected] = useState<Node | null>(null);

	const onNodeClick = useCallback(
		(_: any, node: Node) => setSelected(node),
		[],
	);
	const onPaneClick = useCallback(() => setSelected(null), []);

	// Sync selected separately so dep-wire effect gets clean nodes state
	const updateNode = useCallback(
		(nodeId: string, data: Partial<StackServiceData>) => {
			graphUpdate(nodeId, data);
			setSelected(prev =>
				prev && prev.id === nodeId
					? {...prev, data: {...prev.data, ...data}}
					: prev,
			);
		},
		[graphUpdate],
	);

	const deleteNode = useCallback(
		(nodeId: string) => {
			graphDelete(nodeId);
			setSelected(null);
		},
		[graphDelete],
	);

	return (
		<div
			style={{
				display: 'flex',
				width: '100%',
				height: '100%',
				position: 'relative',
			}}>
			{/* Canvas */}
			<div style={{flex: 1, position: 'relative'}}>
				<ReactFlow
					nodes={nodes}
					edges={edges}
					nodeTypes={nodeTypes}
					edgeTypes={edgeTypes}
					onNodesChange={onNodesChange}
					onEdgesChange={onEdgesChange}
					onNodeClick={onNodeClick}
					onPaneClick={onPaneClick}
					nodesConnectable={false}
					fitView
					fitViewOptions={{padding: 0.3}}
					proOptions={{hideAttribution: true}}
					style={{background: '#0d1117'}}>
					{/* Grid lines — Railway2 style */}
					<Background
						variant={BackgroundVariant.Lines}
						gap={40}
						lineWidth={0.5}
						color="rgba(255,255,255,0.04)"
						style={{background: '#0d1117'}}
					/>
					<Controls
						position="bottom-left"
						style={{
							background: '#161b22',
							border: '1px solid rgba(255,255,255,0.08)',
							borderRadius: 8,
						}}
					/>
				</ReactFlow>

				{/* + button — top right exactly like Railway */}
				<StackAddButton onShow={() => setShowAdd(true)} />

				{nodes.length === 0 && <StackEmptyState />}
			</div>

			{/* Right config panel */}
			{selected && (
				<StackConfigPanel
					node={selected}
					environmentId={environmentId}
					onUpdate={updateNode}
					onDelete={deleteNode}
					onClose={() => setSelected(null)}
				/>
			)}

			{showAdd && (
				<AddServiceModal
					onAdd={addService}
					onClose={() => setShowAdd(false)}
				/>
			)}
		</div>
	);
}

export function StackCanvas({environmentId}: {environmentId: number}) {
	return (
		<ReactFlowProvider>
			<CanvasInner environmentId={environmentId} />
		</ReactFlowProvider>
	);
}
