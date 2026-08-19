import {useState, useMemo} from 'react';
import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {SlidersHorizontal, HardDrive, Globe, Zap} from 'lucide-react';
import {type ComposeService} from './compose-visualizer';
import {
	resolveLocalIcon,
	isValidDockerImage,
} from '#/lib/service-icon-resolver';
import {siDocker} from 'simple-icons';

export type ComposeServiceNodeData = ComposeService & {
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
};

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

	const known = useMemo(() => resolveLocalIcon(image), [image]);

	if (known) {
		const vb = known.viewBox || '0 0 24 24';
		return (
			<div
				style={{
					width: 36,
					height: 36,
					borderRadius: 9,
					flexShrink: 0,
					background: `${known.color}18`,
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
				}}>
				<svg
					viewBox={vb}
					style={{width: 20, height: 20, fill: known.color}}>
					{Array.isArray(known.paths) ? (
						known.paths.map((p, idx) => <path key={idx} d={p} />)
					) : (
						<path d={known.path} />
					)}
				</svg>
			</div>
		);
	}

	if (cleanImageName && !failed) {
		const iconUrl = `https://cdn.jsdelivr.net/gh/walkxcode/dashboard-icons/svg/${cleanImageName}.svg`;
		return (
			<div
				style={{
					width: 36,
					height: 36,
					borderRadius: 9,
					flexShrink: 0,
					background: 'rgba(255,255,255,0.05)',
					border: '1px solid rgba(255,255,255,0.08)',
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					padding: 4,
					overflow: 'hidden',
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

	return (
		<div
			style={{
				width: 36,
				height: 36,
				borderRadius: 9,
				flexShrink: 0,
				background: '#1e90ff18',
				display: 'flex',
				alignItems: 'center',
				justifyContent: 'center',
			}}>
			<svg
				viewBox="0 0 24 24"
				style={{width: 18, height: 18, fill: '#1e90ff'}}>
				<path d={siDocker.path} />
			</svg>
		</div>
	);
}

// ─── Card ─────────────────────────────────────────────────────────────────────

export function ComposeServiceNode({
	data,
	selected,
}: NodeProps<Node<ComposeServiceNodeData>>) {
	const envCount = Object.keys(data.envVars ?? {}).length;
	const shortImg =
		(data.image ?? '').length > 22
			? (data.image ?? '').slice(0, 22) + '…'
			: (data.image ?? 'no image');

	return (
		<div
			style={{
				width: 260,
				borderRadius: 12,
				background: '#161b22',
				border: selected
					? '1px solid rgba(88,166,255,0.55)'
					: '1px solid rgba(255,255,255,0.08)',
				boxShadow: selected
					? '0 0 0 3px rgba(88,166,255,0.1), 0 4px 20px rgba(0,0,0,0.4)'
					: '0 2px 12px rgba(0,0,0,0.3)',
				transition: 'border-color .12s, box-shadow .12s',
				overflow: 'hidden',
				position: 'relative',
			}}>
			{/* Main body */}
			<div style={{padding: '12px 14px 10px'}}>
				{/* Icon + name + action trigger */}
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 10,
						marginBottom: 10,
					}}>
					<ServiceIcon image={data.image} name={data.name} />
					<div style={{flex: 1, minWidth: 0}}>
						<p
							style={{
								margin: 0,
								fontSize: 13.5,
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
								fontFamily: 'monospace',
								color: '#6e7681',
								whiteSpace: 'nowrap',
								overflow: 'hidden',
								textOverflow: 'ellipsis',
							}}>
							{shortImg}
						</p>
					</div>
					<div
						style={{
							padding: '3px 7px',
							borderRadius: 6,
							background: 'rgba(56,189,248,0.12)',
							border: '1px solid rgba(56,189,248,0.25)',
							color: '#38bdf8',
							fontSize: 10,
							fontWeight: 700,
							display: 'flex',
							alignItems: 'center',
							gap: 3,
							flexShrink: 0,
						}}>
						<Zap size={10} /> Actions
					</div>
				</div>

				{/* depends_on badge row */}
				{data.dependsOn.length > 0 && (
					<div
						style={{
							display: 'flex',
							flexWrap: 'wrap',
							gap: 4,
							marginBottom: 8,
						}}>
						{data.dependsOn.map(d => (
							<span
								key={d}
								style={{
									fontSize: 9.5,
									fontWeight: 600,
									padding: '2px 7px',
									borderRadius: 20,
									background: 'rgba(255,255,255,0.05)',
									color: '#8b949e',
									border: '1px solid rgba(255,255,255,0.08)',
								}}>
								↳ {d}
							</span>
						))}
					</div>
				)}

				{/* Config row */}
				<div
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 7,
						marginBottom: envCount > 0 && data.volumes.length > 0 ? 6 : 0,
					}}>
					<SlidersHorizontal size={11} color="#6e7681" />
					<span style={{fontSize: 11, color: '#6e7681'}}>
						{envCount > 0
							? `${envCount} variable${envCount > 1 ? 's' : ''}`
							: 'No config required'}
					</span>
				</div>

				{/* Volume row */}
				{data.volumes.slice(0, 1).map((vol, i) => (
					<div
						key={i}
						style={{
							display: 'flex',
							alignItems: 'center',
							gap: 7,
							marginBottom: 4,
						}}>
						<HardDrive size={11} color="#6e7681" />
						<span
							style={{
								fontSize: 10,
								fontFamily: 'monospace',
								color: '#6e7681',
								whiteSpace: 'nowrap',
								overflow: 'hidden',
								textOverflow: 'ellipsis',
							}}>
							{vol}
						</span>
					</div>
				))}

				{/* Ports row */}
				{data.ports.slice(0, 1).map((port, i) => (
					<div
						key={i}
						style={{display: 'flex', alignItems: 'center', gap: 7}}>
						<Globe size={11} color="#6e7681" />
						<span
							style={{
								fontSize: 10,
								fontFamily: 'monospace',
								color: '#6e7681',
							}}>
							{port}
						</span>
					</div>
				))}
			</div>

			{/* Handles */}
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
