import {Minus, Plus} from 'lucide-react';

const btnStyle = (radius: string): React.CSSProperties => ({
	width: 22,
	height: 22,
	borderRadius: radius,
	background: 'rgba(255,255,255,0.05)',
	border: '1px solid rgba(255,255,255,0.08)',
	color: '#8b949e',
	cursor: 'pointer',
	display: 'flex',
	alignItems: 'center',
	justifyContent: 'center',
	transition: 'background .1s',
});

/** Segmented −/value/+ replica stepper used by stack service cards. */
export function StackReplicaBar({
	value,
	onStep,
}: {
	value: number;
	onStep: (delta: number) => void;
}) {
	const hover = (e: React.MouseEvent<HTMLButtonElement>) =>
		(e.currentTarget.style.background = 'rgba(255,255,255,0.1)');
	const rest = (e: React.MouseEvent<HTMLButtonElement>) =>
		(e.currentTarget.style.background = 'rgba(255,255,255,0.05)');
	const stop = (delta: number) => (e: React.MouseEvent) => {
		e.stopPropagation();
		onStep(delta);
	};

	return (
		<div
			style={{
				display: 'flex',
				alignItems: 'center',
				justifyContent: 'space-between',
				padding: '8px 16px',
				background: 'rgba(0,0,0,0.2)',
				borderTop: '1px solid rgba(255,255,255,0.05)',
			}}>
			<span style={{fontSize: 11, color: '#6e7681', fontWeight: 500}}>
				Replicas
			</span>
			<div style={{display: 'flex', alignItems: 'center', gap: 0}}>
				<button
					type="button"
					onClick={stop(-1)}
					onMouseEnter={hover}
					onMouseLeave={rest}
					style={btnStyle('6px 0 0 6px')}>
					<Minus size={11} />
				</button>
				<div
					style={{
						minWidth: 28,
						height: 22,
						textAlign: 'center',
						background: 'rgba(255,255,255,0.04)',
						border: '1px solid rgba(255,255,255,0.08)',
						borderLeft: 'none',
						borderRight: 'none',
						fontSize: 12,
						fontWeight: 600,
						color: '#e6edf3',
						display: 'flex',
						alignItems: 'center',
						justifyContent: 'center',
						padding: '0 6px',
					}}>
					{value}
				</div>
				<button
					type="button"
					onClick={stop(1)}
					onMouseEnter={hover}
					onMouseLeave={rest}
					style={btnStyle('0 6px 6px 0')}>
					<Plus size={11} />
				</button>
			</div>
		</div>
	);
}
