import {useState, useEffect, useRef} from 'react';
import {type Node} from '@xyflow/react';
import {X, Trash2, Plus, Minus} from 'lucide-react';
import {type StackServiceData, type EnvVar} from './stack-service-card';

interface StackConfigPanelProps {
	node: Node;
	environmentId: number;
	onUpdate: (nodeId: string, data: Partial<StackServiceData>) => void;
	onDelete: (nodeId: string) => void;
	onClose: () => void;
}

function SectionLabel({children}: {children: React.ReactNode}) {
	return (
		<label
			style={{
				fontSize: 10.5,
				fontWeight: 600,
				color: '#6e7681',
				textTransform: 'uppercase',
				letterSpacing: '0.06em',
				display: 'block',
				marginBottom: 6,
			}}>
			{children}
		</label>
	);
}

// Uncontrolled-style input — local value, flush to parent on blur
function LiveInput({
	value,
	onChange,
	placeholder,
	mono,
	wide,
}: {
	value: string;
	onChange: (v: string) => void;
	placeholder?: string;
	mono?: boolean;
	wide?: boolean;
}) {
	const [local, setLocal] = useState(value);
	const ref = useRef(false); // is focused?

	// Sync from parent only when not focused (i.e., external update)
	useEffect(() => {
		if (!ref.current) setLocal(value);
	}, [value]);

	return (
		<input
			value={local}
			onChange={e => setLocal(e.target.value)}
			onFocus={() => {
				ref.current = true;
			}}
			onBlur={e => {
				ref.current = false;
				onChange(e.target.value); // flush to parent on blur
			}}
			placeholder={placeholder}
			style={{
				width: wide ? '100%' : undefined,
				flex: wide ? undefined : 1,
				padding: '6px 8px',
				borderRadius: 6,
				background: '#0d1117',
				border: '1px solid rgba(255,255,255,0.08)',
				color: '#e6edf3',
				fontSize: mono ? 11 : 12.5,
				fontFamily: mono ? 'monospace' : 'inherit',
				outline: 'none',
				boxSizing: 'border-box',
				transition: 'border-color .12s',
			}}
			onMouseEnter={e =>
				(e.currentTarget.style.borderColor = 'rgba(255,255,255,0.15)')
			}
			onMouseLeave={e => {
				if (document.activeElement !== e.currentTarget)
					e.currentTarget.style.borderColor = 'rgba(255,255,255,0.08)';
			}}
		/>
	);
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

	const [newVol, setNewVol] = useState('');
	const [newPort, setNewPort] = useState('');
	const [newKey, setNewKey] = useState('');
	const [newVal, setNewVal] = useState('');

	/* ── Env var helpers ── */
	const updateEnvKey = (i: number, k: string) =>
		onUpdate(node.id, {
			envVars: envVars.map((ev, idx) =>
				idx === i ? {...ev, key: k} : ev,
			),
		});

	const updateEnvVal = (i: number, v: string) =>
		onUpdate(node.id, {
			envVars: envVars.map((ev, idx) =>
				idx === i ? {...ev, value: v} : ev,
			),
		});

	const removeEnvVar = (i: number) =>
		onUpdate(node.id, {envVars: envVars.filter((_, idx) => idx !== i)});

	const addEnvVar = () => {
		if (!newKey.trim()) return;
		onUpdate(node.id, {
			envVars: [...envVars, {key: newKey.trim(), value: newVal.trim()}],
		});
		setNewKey('');
		setNewVal('');
	};

	/* ── Volume helpers ── */
	const addVol = () => {
		if (newVol.trim()) {
			onUpdate(node.id, {volumes: [...volumes, newVol.trim()]});
			setNewVol('');
		}
	};
	const rmVol = (i: number) =>
		onUpdate(node.id, {volumes: volumes.filter((_, j) => j !== i)});

	/* ── Port helpers ── */
	const addPort = () => {
		if (newPort.trim()) {
			onUpdate(node.id, {ports: [...ports, newPort.trim()]});
			setNewPort('');
		}
	};
	const rmPort = (i: number) =>
		onUpdate(node.id, {ports: ports.filter((_, j) => j !== i)});

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
				{/* Name & Image */}
				<div style={{display: 'flex', flexDirection: 'column', gap: 10}}>
					<div>
						<SectionLabel>Service Name</SectionLabel>
						<LiveInput
							value={data.name}
							wide
							placeholder="my-service"
							onChange={v => onUpdate(node.id, {name: v})}
						/>
					</div>
					<div>
						<SectionLabel>Image</SectionLabel>
						<LiveInput
							value={data.image}
							wide
							mono
							placeholder="nginx:alpine"
							onChange={v => onUpdate(node.id, {image: v})}
						/>
					</div>
				</div>

				{/* Replicas */}
				<div>
					<SectionLabel>Replicas</SectionLabel>
					<div style={{display: 'flex', alignItems: 'center', gap: 8}}>
						<button
							type="button"
							onClick={() =>
								onUpdate(node.id, {
									replicas: Math.max(1, (data.replicas ?? 1) - 1),
								})
							}
							style={{
								width: 28,
								height: 28,
								borderRadius: 6,
								background: '#21262d',
								border: '1px solid rgba(255,255,255,0.08)',
								cursor: 'pointer',
								color: '#8b949e',
								display: 'flex',
								alignItems: 'center',
								justifyContent: 'center',
							}}>
							<Minus size={11} />
						</button>
						<span
							style={{
								fontSize: 15,
								fontWeight: 700,
								color: '#e6edf3',
								minWidth: 24,
								textAlign: 'center',
							}}>
							{data.replicas ?? 1}
						</span>
						<button
							type="button"
							onClick={() =>
								onUpdate(node.id, {replicas: (data.replicas ?? 1) + 1})
							}
							style={{
								width: 28,
								height: 28,
								borderRadius: 6,
								background: '#21262d',
								border: '1px solid rgba(255,255,255,0.08)',
								cursor: 'pointer',
								color: '#8b949e',
								display: 'flex',
								alignItems: 'center',
								justifyContent: 'center',
							}}>
							<Plus size={11} />
						</button>
						<span style={{fontSize: 11, color: '#6e7681'}}>
							replica{(data.replicas ?? 1) > 1 ? 's' : ''}
						</span>
					</div>
				</div>

				{/* ── Environment Variables ── */}
				<div>
					<SectionLabel>Environment Variables</SectionLabel>

					{/* Syntax hint */}
					<div
						style={{
							marginBottom: 8,
							padding: '6px 9px',
							borderRadius: 6,
							background: 'rgba(88,166,255,0.05)',
							border: '1px solid rgba(88,166,255,0.1)',
						}}>
						<p
							style={{
								margin: '0 0 2px',
								fontSize: 10,
								fontWeight: 600,
								color: '#58a6ff',
							}}>
							Reference another service:
						</p>
						<code
							style={{
								fontSize: 10,
								color: '#8b949e',
								fontFamily: 'monospace',
								display: 'block',
							}}>
							{'${{ServiceName.VARIABLE}}'}
						</code>
						<p style={{margin: '2px 0 0', fontSize: 10, color: '#6e7681'}}>
							→ auto-wires dependency edge
						</p>
					</div>

					{/* Existing env vars */}
					<div style={{display: 'flex', flexDirection: 'column', gap: 4}}>
						{envVars.map((ev, i) => (
							<div
								key={i}
								style={{display: 'flex', gap: 4, alignItems: 'center'}}>
								{/* KEY */}
								<LiveInput
									value={ev.key}
									mono
									placeholder="KEY"
									onChange={k => updateEnvKey(i, k)}
								/>
								<span
									style={{color: '#30363d', fontSize: 12, flexShrink: 0}}>
									=
								</span>
								{/* VALUE — blur triggers dep-wire scan */}
								<LiveInput
									value={ev.value}
									mono
									placeholder="value or ${{Svc.VAR}}"
									onChange={v => updateEnvVal(i, v)}
								/>
								<button
									type="button"
									onClick={() => removeEnvVar(i)}
									style={{
										width: 22,
										height: 22,
										borderRadius: 5,
										background: 'transparent',
										border: '1px solid rgba(248,81,73,0.25)',
										cursor: 'pointer',
										color: '#f85149',
										display: 'flex',
										alignItems: 'center',
										justifyContent: 'center',
										flexShrink: 0,
									}}>
									<Minus size={9} />
								</button>
							</div>
						))}

						{/* Add row */}
						<div
							style={{
								display: 'flex',
								gap: 4,
								alignItems: 'center',
								marginTop: 4,
							}}>
							<input
								value={newKey}
								onChange={e => setNewKey(e.target.value)}
								placeholder="KEY"
								onKeyDown={e => e.key === 'Enter' && addEnvVar()}
								style={{
									width: 90,
									padding: '6px 8px',
									borderRadius: 6,
									background: '#0d1117',
									border: '1px dashed rgba(255,255,255,0.12)',
									color: '#e6edf3',
									fontSize: 11,
									fontFamily: 'monospace',
									outline: 'none',
									boxSizing: 'border-box',
								}}
							/>
							<span
								style={{color: '#30363d', fontSize: 12, flexShrink: 0}}>
								=
							</span>
							<input
								value={newVal}
								onChange={e => setNewVal(e.target.value)}
								placeholder="value"
								onKeyDown={e => e.key === 'Enter' && addEnvVar()}
								style={{
									flex: 1,
									padding: '6px 8px',
									borderRadius: 6,
									background: '#0d1117',
									border: '1px dashed rgba(255,255,255,0.12)',
									color: '#8b949e',
									fontSize: 11,
									fontFamily: 'monospace',
									outline: 'none',
									boxSizing: 'border-box',
								}}
							/>
							<button
								type="button"
								onClick={addEnvVar}
								style={{
									width: 22,
									height: 22,
									borderRadius: 5,
									background: 'rgba(255,255,255,0.05)',
									border: '1px solid rgba(255,255,255,0.1)',
									cursor: 'pointer',
									color: '#6e7681',
									display: 'flex',
									alignItems: 'center',
									justifyContent: 'center',
									flexShrink: 0,
								}}>
								<Plus size={9} />
							</button>
						</div>
					</div>
				</div>

				{/* ── Volumes ── */}
				<div>
					<SectionLabel>Volumes</SectionLabel>
					{volumes.map((vol, i) => (
						<div
							key={i}
							style={{display: 'flex', gap: 5, marginBottom: 4}}>
							<span
								style={{
									flex: 1,
									fontSize: 10.5,
									color: '#8b949e',
									fontFamily: 'monospace',
									background: '#0d1117',
									padding: '5px 8px',
									borderRadius: 5,
									border: '1px solid rgba(255,255,255,0.06)',
									whiteSpace: 'nowrap',
									overflow: 'hidden',
									textOverflow: 'ellipsis',
								}}>
								{vol}
							</span>
							<button
								type="button"
								onClick={() => rmVol(i)}
								style={{
									width: 22,
									height: 22,
									borderRadius: 5,
									background: 'transparent',
									border: '1px solid rgba(248,81,73,0.25)',
									cursor: 'pointer',
									color: '#f85149',
									display: 'flex',
									alignItems: 'center',
									justifyContent: 'center',
									flexShrink: 0,
								}}>
								<Minus size={9} />
							</button>
						</div>
					))}
					<div style={{display: 'flex', gap: 5, marginTop: 4}}>
						<input
							value={newVol}
							onChange={e => setNewVol(e.target.value)}
							placeholder="/data/path"
							onKeyDown={e => {
								if (e.key === 'Enter') addVol();
							}}
							style={{
								flex: 1,
								padding: '5px 8px',
								borderRadius: 5,
								background: '#0d1117',
								border: '1px dashed rgba(255,255,255,0.1)',
								color: '#8b949e',
								fontSize: 10.5,
								fontFamily: 'monospace',
								outline: 'none',
								boxSizing: 'border-box',
							}}
						/>
						<button
							type="button"
							onClick={addVol}
							style={{
								width: 26,
								height: 26,
								borderRadius: 5,
								background: '#21262d',
								border: '1px solid rgba(255,255,255,0.08)',
								cursor: 'pointer',
								color: '#6e7681',
								display: 'flex',
								alignItems: 'center',
								justifyContent: 'center',
							}}>
							<Plus size={11} />
						</button>
					</div>
				</div>

				{/* ── Ports ── */}
				<div>
					<SectionLabel>Ports</SectionLabel>
					{ports.map((port, i) => (
						<div
							key={i}
							style={{display: 'flex', gap: 5, marginBottom: 4}}>
							<span
								style={{
									flex: 1,
									fontSize: 10.5,
									color: '#8b949e',
									fontFamily: 'monospace',
									background: '#0d1117',
									padding: '5px 8px',
									borderRadius: 5,
									border: '1px solid rgba(255,255,255,0.06)',
								}}>
								{port}
							</span>
							<button
								type="button"
								onClick={() => rmPort(i)}
								style={{
									width: 22,
									height: 22,
									borderRadius: 5,
									background: 'transparent',
									border: '1px solid rgba(248,81,73,0.25)',
									cursor: 'pointer',
									color: '#f85149',
									display: 'flex',
									alignItems: 'center',
									justifyContent: 'center',
									flexShrink: 0,
								}}>
								<Minus size={9} />
							</button>
						</div>
					))}
					<div style={{display: 'flex', gap: 5, marginTop: 4}}>
						<input
							value={newPort}
							onChange={e => setNewPort(e.target.value)}
							placeholder="8080:80"
							onKeyDown={e => {
								if (e.key === 'Enter') addPort();
							}}
							style={{
								flex: 1,
								padding: '5px 8px',
								borderRadius: 5,
								background: '#0d1117',
								border: '1px dashed rgba(255,255,255,0.1)',
								color: '#8b949e',
								fontSize: 10.5,
								fontFamily: 'monospace',
								outline: 'none',
								boxSizing: 'border-box',
							}}
						/>
						<button
							type="button"
							onClick={addPort}
							style={{
								width: 26,
								height: 26,
								borderRadius: 5,
								background: '#21262d',
								border: '1px solid rgba(255,255,255,0.08)',
								cursor: 'pointer',
								color: '#6e7681',
								display: 'flex',
								alignItems: 'center',
								justifyContent: 'center',
							}}>
							<Plus size={11} />
						</button>
					</div>
				</div>
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
