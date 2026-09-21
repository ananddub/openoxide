import {useState} from 'react';
import {
	Handle,
	Position,
	type NodeProps,
	type Node,
	useReactFlow,
} from '@xyflow/react';
import {SlidersHorizontal, HardDrive} from 'lucide-react';
import {DockerServiceIcon} from '../common/docker-service-icon';
import {StackReplicaBar} from './stack-replica-bar';

// ─── Types ────────────────────────────────────────────────────────────────────

export type EnvVar = {key: string; value: string};

export type StackServiceData = {
	name: string;
	image: string;
	replicas: number;
	envVars: EnvVar[];
	volumes: string[];
	ports: string[];
	isNew?: boolean;
};

// ─── Card ─────────────────────────────────────────────────────────────────────

export function StackServiceCard({
	data,
	selected,
	id,
}: NodeProps<Node<StackServiceData>>) {
	const [hov, setHov] = useState(false);
	const {setNodes} = useReactFlow();

	const setReplicas = (delta: number) => {
		setNodes((ns: any[]) =>
			ns.map((n: any) =>
				n.id === id
					? {
							...n,
							data: {
								...n.data,
								replicas: Math.max(1, (n.data.replicas ?? 1) + delta),
							},
						}
					: n,
			),
		);
	};

	const shortImage =
		data.image.length > 30 ? data.image.slice(0, 30) + '…' : data.image;
	const volumes = data.volumes ?? [];

	return (
		<div
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: 280,
				borderRadius: 12,
				background: '#161b22',
				border: selected
					? '1px solid rgba(88,166,255,0.55)'
					: hov
						? '1px solid rgba(255,255,255,0.16)'
						: '1px solid rgba(255,255,255,0.08)',
				boxShadow: selected
					? '0 0 0 3px rgba(88,166,255,0.1), 0 4px 20px rgba(0,0,0,0.4)'
					: '0 2px 12px rgba(0,0,0,0.3)',
				transition: 'border-color .12s, box-shadow .12s',
				overflow: 'hidden',
				userSelect: 'none',
				cursor: 'pointer',
			}}>
			{/* ── Main body ─────────────────────────────────────── */}
			<div style={{padding: '16px 16px 12px'}}>
				{/* Icon + name row */}
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 12,
						marginBottom: 14,
					}}>
					<DockerServiceIcon image={data.image} size={38} />
					<div style={{flex: 1, minWidth: 0}}>
						<p
							style={{
								margin: 0,
								fontSize: 15,
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
								margin: '3px 0 0',
								fontSize: 11,
								color: '#6e7681',
								whiteSpace: 'nowrap',
								overflow: 'hidden',
								textOverflow: 'ellipsis',
								fontFamily: 'monospace',
							}}>
							{shortImage}
						</p>
					</div>
				</div>

				{/* Config row */}
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 8,
						marginBottom: 8,
					}}>
					<SlidersHorizontal size={13} color="#6e7681" />
					<span style={{fontSize: 12, color: '#6e7681'}}>
						{(data.envVars ?? []).length > 0
							? `${data.envVars.length} variable${data.envVars.length > 1 ? 's' : ''}`
							: 'No config required'}
					</span>
				</div>

				{/* Volume rows */}
				{volumes.slice(0, 2).map((vol, i) => (
					<div
						key={i}
						style={{
							display: 'flex',
							alignItems: 'center',
							gap: 8,
							marginBottom: 4,
						}}>
						<HardDrive size={12} color="#6e7681" />
						<span
							style={{
								fontSize: 11,
								color: '#6e7681',
								fontFamily: 'monospace',
								whiteSpace: 'nowrap',
								overflow: 'hidden',
								textOverflow: 'ellipsis',
							}}>
							{vol}
						</span>
					</div>
				))}
			</div>

			<StackReplicaBar value={data.replicas ?? 1} onStep={setReplicas} />

			{/* Handles — all 4 sides */}
			<Handle
				type="target"
				position={Position.Left}
				style={{
					width: 8,
					height: 8,
					background: '#30363d',
					border: '2px solid #0d1117',
					left: -5,
				}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{
					width: 8,
					height: 8,
					background: '#30363d',
					border: '2px solid #0d1117',
					right: -5,
				}}
			/>
		</div>
	);
}
