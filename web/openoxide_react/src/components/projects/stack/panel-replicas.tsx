import {Plus, Minus} from 'lucide-react';
import {SectionLabel} from './panel-primitives';

const stepBtn: React.CSSProperties = {
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
};

export function PanelReplicas({
	value,
	onStep,
}: {
	value: number;
	onStep: (delta: number) => void;
}) {
	return (
		<div>
			<SectionLabel>Replicas</SectionLabel>
			<div style={{display: 'flex', alignItems: 'center', gap: 8}}>
				<button type="button" onClick={() => onStep(-1)} style={stepBtn}>
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
					{value}
				</span>
				<button type="button" onClick={() => onStep(1)} style={stepBtn}>
					<Plus size={11} />
				</button>
				<span style={{fontSize: 11, color: '#6e7681'}}>
					replica{value > 1 ? 's' : ''}
				</span>
			</div>
		</div>
	);
}
