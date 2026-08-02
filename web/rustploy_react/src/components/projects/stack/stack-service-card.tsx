import {useState, useMemo} from 'react';
import {Handle, Position, type NodeProps, type Node, useReactFlow} from '@xyflow/react';
import {siDocker} from 'simple-icons';
import {SlidersHorizontal, HardDrive, Minus, Plus} from 'lucide-react';
import {resolveLocalIcon, isValidDockerImage} from '#/lib/service-icon-resolver';

// ─── 3-Tier Hybrid Icon Resolver ──────────────────────────────────────────────

function ServiceIcon({image}: {image: string; name?: string}) {
	const [failed, setFailed] = useState(false);

	const cleanImageName = useMemo(() => {
		if (!isValidDockerImage(image)) return '';
		let n = image.split(':')[0].split('@')[0];
		if (n.includes('/')) {
			const parts = n.split('/');
			n = parts[parts.length - 1];
		}
		return n.toLowerCase().trim();
	}, [image]);

	// Tier 1: Auto-match valid Docker image against 3,450+ local simple-icons + Redpanda
	const known = useMemo(() => resolveLocalIcon(image), [image]);

	if (known) {
		const vb = known.viewBox || '0 0 24 24';
		return (
			<div style={{
				width: 38, height: 38, borderRadius: 9, flexShrink: 0,
				background: `${known.color}18`,
				display: 'flex', alignItems: 'center', justifyContent: 'center',
			}}>
				<svg viewBox={vb} style={{width: 20, height: 20, fill: known.color}}>
					{Array.isArray(known.paths) ? (
						known.paths.map((p, idx) => <path key={idx} d={p} />)
					) : (
						<path d={known.path} />
					)}
				</svg>
			</div>
		);
	}

	// Tier 2: Dynamic fetch for custom Docker Hub images (ONLY if valid image tag)
	if (cleanImageName && !failed) {
		const iconUrl = `https://cdn.jsdelivr.net/gh/walkxcode/dashboard-icons/svg/${cleanImageName}.svg`;
		return (
			<div style={{
				width: 38, height: 38, borderRadius: 9, flexShrink: 0,
				background: 'rgba(255,255,255,0.05)',
				border: '1px solid rgba(255,255,255,0.08)',
				display: 'flex', alignItems: 'center', justifyContent: 'center',
				padding: 4, overflow: 'hidden',
			}}>
				<img
					src={iconUrl}
					alt={cleanImageName}
					onError={() => setFailed(true)}
					style={{width: '100%', height: '100%', objectFit: 'contain'}}
				/>
			</div>
		);
	}

	// Tier 3: Fallback to Docker 🐳 Logo for local builds, '.', or unknown images
	return (
		<div style={{
			width: 38, height: 38, borderRadius: 9, flexShrink: 0,
			background: '#1e90ff18',
			display: 'flex', alignItems: 'center', justifyContent: 'center',
		}}>
			<svg viewBox="0 0 24 24" style={{width: 20, height: 20, fill: '#1e90ff'}}>
				<path d={siDocker.path} />
			</svg>
		</div>
	);
}

// ─── Types ────────────────────────────────────────────────────────────────────

export type EnvVar = {key: string; value: string};

export type StackServiceData = {
	name:     string;
	image:    string;
	replicas: number;
	envVars:  EnvVar[];
	volumes:  string[];
	ports:    string[];
	isNew?:   boolean;
};

// ─── Card ─────────────────────────────────────────────────────────────────────

export function StackServiceCard({data, selected, id}: NodeProps<Node<StackServiceData>>) {
	const [hov, setHov] = useState(false);
	const {setNodes} = useReactFlow();

	const setReplicas = (delta: number) => {
		setNodes((ns: any[]) => ns.map((n: any) =>
			n.id === id
				? {...n, data: {...n.data, replicas: Math.max(1, (n.data.replicas ?? 1) + delta)}}
				: n,
		));
	};

	const shortImage = data.image.length > 30 ? data.image.slice(0, 30) + '…' : data.image;
	const volumes    = data.volumes ?? [];

	return (
		<div
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				width: 280, borderRadius: 12,
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
				overflow: 'hidden', userSelect: 'none', cursor: 'pointer',
			}}
		>
			{/* ── Main body ─────────────────────────────────────── */}
			<div style={{padding: '16px 16px 12px'}}>
				{/* Icon + name row */}
				<div style={{display: 'flex', alignItems: 'center', gap: 12, marginBottom: 14}}>
					<ServiceIcon image={data.image} name={data.name} />
					<div style={{flex: 1, minWidth: 0}}>
						<p style={{
							margin: 0, fontSize: 15, fontWeight: 700, color: '#e6edf3',
							whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
						}}>
							{data.name}
						</p>
						<p style={{
							margin: '3px 0 0', fontSize: 11, color: '#6e7681',
							whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
							fontFamily: 'monospace',
						}}>
							{shortImage}
						</p>
					</div>
				</div>

				{/* Config row */}
				<div style={{display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8}}>
					<SlidersHorizontal size={13} color="#6e7681" />
					<span style={{fontSize: 12, color: '#6e7681'}}>
						{(data.envVars ?? []).length > 0
							? `${data.envVars.length} variable${data.envVars.length > 1 ? 's' : ''}`
							: 'No config required'}
					</span>
				</div>

				{/* Volume rows */}
				{volumes.slice(0, 2).map((vol, i) => (
					<div key={i} style={{display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4}}>
						<HardDrive size={12} color="#6e7681" />
						<span style={{fontSize: 11, color: '#6e7681', fontFamily: 'monospace',
							whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>
							{vol}
						</span>
					</div>
				))}
			</div>

			{/* ── Replica bar ───────────────────────────────────── */}
			<div style={{
				display: 'flex', alignItems: 'center', justifyContent: 'space-between',
				padding: '8px 16px',
				background: 'rgba(0,0,0,0.2)',
				borderTop: '1px solid rgba(255,255,255,0.05)',
			}}>
				<span style={{fontSize: 11, color: '#6e7681', fontWeight: 500}}>
					Replicas
				</span>
				<div style={{display: 'flex', alignItems: 'center', gap: 0}}>
					<button type="button"
						onClick={e => { e.stopPropagation(); setReplicas(-1); }}
						style={{
							width: 22, height: 22, borderRadius: '6px 0 0 6px',
							background: 'rgba(255,255,255,0.05)',
							border: '1px solid rgba(255,255,255,0.08)',
							color: '#8b949e', cursor: 'pointer', display: 'flex',
							alignItems: 'center', justifyContent: 'center',
							transition: 'background .1s',
						}}
						onMouseEnter={e => (e.currentTarget.style.background = 'rgba(255,255,255,0.1)')}
						onMouseLeave={e => (e.currentTarget.style.background = 'rgba(255,255,255,0.05)')}
					>
						<Minus size={11} />
					</button>
					<div style={{
						minWidth: 28, height: 22, textAlign: 'center',
						background: 'rgba(255,255,255,0.04)',
						border: '1px solid rgba(255,255,255,0.08)',
						borderLeft: 'none', borderRight: 'none',
						fontSize: 12, fontWeight: 600, color: '#e6edf3',
						display: 'flex', alignItems: 'center', justifyContent: 'center',
						padding: '0 6px',
					}}>
						{data.replicas ?? 1}
					</div>
					<button type="button"
						onClick={e => { e.stopPropagation(); setReplicas(+1); }}
						style={{
							width: 22, height: 22, borderRadius: '0 6px 6px 0',
							background: 'rgba(255,255,255,0.05)',
							border: '1px solid rgba(255,255,255,0.08)',
							color: '#8b949e', cursor: 'pointer', display: 'flex',
							alignItems: 'center', justifyContent: 'center',
							transition: 'background .1s',
						}}
						onMouseEnter={e => (e.currentTarget.style.background = 'rgba(255,255,255,0.1)')}
						onMouseLeave={e => (e.currentTarget.style.background = 'rgba(255,255,255,0.05)')}
					>
						<Plus size={11} />
					</button>
				</div>
			</div>

			{/* Handles — all 4 sides */}
			<Handle type="target" position={Position.Left}
				style={{width: 8, height: 8, background: '#30363d', border: '2px solid #0d1117', left: -5}} />
			<Handle type="source" position={Position.Right}
				style={{width: 8, height: 8, background: '#30363d', border: '2px solid #0d1117', right: -5}} />
		</div>
	);
}
