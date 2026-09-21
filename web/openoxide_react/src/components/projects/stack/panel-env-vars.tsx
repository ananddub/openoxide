import {useState} from 'react';
import {Plus, Minus} from 'lucide-react';
import type {EnvVar} from './stack-service-card';
import {SectionLabel, LiveInput, iconBtnStyle} from './panel-primitives';

const addInputStyle = (color: string): React.CSSProperties => ({
	padding: '6px 8px',
	borderRadius: 6,
	background: '#0d1117',
	border: '1px dashed rgba(255,255,255,0.12)',
	color,
	fontSize: 11,
	fontFamily: 'monospace',
	outline: 'none',
	boxSizing: 'border-box',
});

export function PanelEnvVars({
	envVars,
	onUpdate,
}: {
	envVars: EnvVar[];
	/** Replaces the whole list (immutable update pattern of the panel). */
	onUpdate: (next: EnvVar[]) => void;
}) {
	const [newKey, setNewKey] = useState('');
	const [newVal, setNewVal] = useState('');

	const setAt = (i: number, patch: Partial<EnvVar>) =>
		onUpdate(
			envVars.map((ev, idx) => (idx === i ? {...ev, ...patch} : ev)),
		);

	const remove = (i: number) =>
		onUpdate(envVars.filter((_, idx) => idx !== i));

	const add = () => {
		if (!newKey.trim()) return;
		onUpdate([...envVars, {key: newKey.trim(), value: newVal.trim()}]);
		setNewKey('');
		setNewVal('');
	};

	return (
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
							onChange={k => setAt(i, {key: k})}
						/>
						<span style={{color: '#30363d', fontSize: 12, flexShrink: 0}}>
							=
						</span>
						{/* VALUE — blur triggers dep-wire scan */}
						<LiveInput
							value={ev.value}
							mono
							placeholder="value or ${{Svc.VAR}}"
							onChange={v => setAt(i, {value: v})}
						/>
						<button
							type="button"
							onClick={() => remove(i)}
							style={iconBtnStyle('#f85149', 'rgba(248,81,73,0.25)')}>
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
						onKeyDown={e => e.key === 'Enter' && add()}
						style={{...addInputStyle('#e6edf3'), width: 90}}
					/>
					<span style={{color: '#30363d', fontSize: 12, flexShrink: 0}}>
						=
					</span>
					<input
						value={newVal}
						onChange={e => setNewVal(e.target.value)}
						placeholder="value"
						onKeyDown={e => e.key === 'Enter' && add()}
						style={{...addInputStyle('#8b949e'), flex: 1}}
					/>
					<button
						type="button"
						onClick={add}
						style={iconBtnStyle('#6e7681', 'rgba(255,255,255,0.1)')}>
						<Plus size={9} />
					</button>
				</div>
			</div>
		</div>
	);
}
