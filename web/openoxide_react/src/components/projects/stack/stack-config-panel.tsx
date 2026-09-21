import {type Node} from '@xyflow/react';
import {X, Trash2} from 'lucide-react';
import {type StackServiceData, type EnvVar} from './stack-service-card';
import {SectionLabel, LiveInput} from './panel-primitives';
import {PanelReplicas} from './panel-replicas';
import {PanelEnvVars} from './panel-env-vars';
import {PanelItemList} from './panel-item-list';

interface StackConfigPanelProps {
	node: Node;
	environmentId: number;
	onUpdate: (nodeId: string, data: Partial<StackServiceData>) => void;
	onDelete: (nodeId: string) => void;
	onClose: () => void;
}

export function StackConfigPanel({
	node,
	onUpdate,
	onDelete,
	onClose,
}: StackConfigPanelProps) {
	const data = node.data as StackServiceData;
	const envVars = (data.envVars ?? []) as EnvVar[];
	const volumes = data.volumes ?? [];
	const ports = data.ports ?? [];

	const patch = (d: Partial<StackServiceData>) => onUpdate(node.id, d);

	return (
		<div
			style={{
				width: 300,
				flexShrink: 0,
				background: '#161b22',
				borderLeft: '1px solid rgba(255,255,255,0.07)',
				display: 'flex',
				flexDirection: 'column',
				overflow: 'hidden',
			}}>
			{/* ── Header ── */}
			<div
				style={{
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'space-between',
					padding: '14px 16px',
					borderBottom: '1px solid rgba(255,255,255,0.07)',
					flexShrink: 0,
				}}>
				<div style={{minWidth: 0}}>
					<p
						style={{
							margin: 0,
							fontSize: 13,
							fontWeight: 700,
							color: '#e6edf3',
							whiteSpace: 'nowrap',
							overflow: 'hidden',
							textOverflow: 'ellipsis',
						}}>
						{data.name}
					</p>
					<p
						style={{
							margin: '2px 0 0',
							fontSize: 10,
							color: '#6e7681',
							fontFamily: 'monospace',
							whiteSpace: 'nowrap',
							overflow: 'hidden',
							textOverflow: 'ellipsis',
						}}>
						{data.image}
					</p>
				</div>
				<button
					type="button"
					onClick={onClose}
					style={{
						background: 'transparent',
						border: 'none',
						cursor: 'pointer',
						color: '#6e7681',
						display: 'flex',
						flexShrink: 0,
						marginLeft: 8,
					}}
					onMouseEnter={e => (e.currentTarget.style.color = '#e6edf3')}
					onMouseLeave={e => (e.currentTarget.style.color = '#6e7681')}>
					<X size={15} />
				</button>
			</div>

			{/* ── Scrollable body ── */}
			<div
				style={{
					flex: 1,
					overflowY: 'auto',
					padding: '14px 14px',
					display: 'flex',
					flexDirection: 'column',
					gap: 18,
				}}>
				<div style={{display: 'flex', flexDirection: 'column', gap: 10}}>
					<div>
						<SectionLabel>Service Name</SectionLabel>
						<LiveInput
							value={data.name}
							wide
							placeholder="my-service"
							onChange={v => patch({name: v})}
						/>
					</div>
					<div>
						<SectionLabel>Image</SectionLabel>
						<LiveInput
							value={data.image}
							wide
							mono
							placeholder="nginx:alpine"
							onChange={v => patch({image: v})}
						/>
					</div>
				</div>

				<PanelReplicas
					value={data.replicas ?? 1}
					onStep={d =>
						patch({replicas: Math.max(1, (data.replicas ?? 1) + d)})
					}
				/>

				<PanelEnvVars
					envVars={envVars}
					onUpdate={next => patch({envVars: next})}
				/>

				<PanelItemList
					label="Volumes"
					items={volumes}
					placeholder="/data/path"
					onAdd={v => patch({volumes: [...volumes, v]})}
					onRemove={i =>
						patch({volumes: volumes.filter((_, j) => j !== i)})
					}
				/>

				<PanelItemList
					label="Ports"
					items={ports}
					placeholder="8080:80"
					onAdd={v => patch({ports: [...ports, v]})}
					onRemove={i => patch({ports: ports.filter((_, j) => j !== i)})}
				/>
			</div>

			{/* ── Footer ── */}
			<div
				style={{
					padding: '12px 14px',
					borderTop: '1px solid rgba(255,255,255,0.07)',
					flexShrink: 0,
				}}>
				<button
					type="button"
					onClick={() => onDelete(node.id)}
					style={{
						display: 'flex',
						alignItems: 'center',
						justifyContent: 'center',
						gap: 6,
						width: '100%',
						padding: '7px 0',
						borderRadius: 8,
						background: 'transparent',
						border: '1px solid rgba(248,81,73,0.3)',
						color: '#f85149',
						fontSize: 12.5,
						fontWeight: 500,
						cursor: 'pointer',
						transition: 'background .12s',
					}}
					onMouseEnter={e =>
						(e.currentTarget.style.background = 'rgba(248,81,73,0.08)')
					}
					onMouseLeave={e =>
						(e.currentTarget.style.background = 'transparent')
					}>
					<Trash2 size={13} /> Remove Service
				</button>
			</div>
		</div>
	);
}
