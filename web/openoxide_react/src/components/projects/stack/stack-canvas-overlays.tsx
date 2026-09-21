import {Plus} from 'lucide-react';

/** Top-right "+" trigger button, Railway style. */
export function StackAddButton({onShow}: {onShow: () => void}) {
	return (
		<div style={{position: 'absolute', top: 14, right: 14, zIndex: 30}}>
			<button
				type="button"
				onClick={onShow}
				style={{
					width: 34,
					height: 34,
					borderRadius: 8,
					background: '#21262d',
					border: '1px solid rgba(255,255,255,0.1)',
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					cursor: 'pointer',
					color: '#e6edf3',
					transition: 'all .12s',
				}}
				onMouseEnter={e => {
					e.currentTarget.style.background = '#30363d';
					e.currentTarget.style.borderColor = 'rgba(255,255,255,0.2)';
				}}
				onMouseLeave={e => {
					e.currentTarget.style.background = '#21262d';
					e.currentTarget.style.borderColor = 'rgba(255,255,255,0.1)';
				}}
				title="Add service">
				<Plus size={16} />
			</button>
		</div>
	);
}

export function StackEmptyState() {
	return (
		<div
			style={{
				position: 'absolute',
				inset: 0,
				display: 'flex',
				flexDirection: 'column',
				alignItems: 'center',
				justifyContent: 'center',
				pointerEvents: 'none',
				zIndex: 5,
				gap: 10,
			}}>
			<p
				style={{
					color: 'rgba(255,255,255,0.18)',
					fontSize: 14,
					fontWeight: 500,
					margin: 0,
				}}>
				Your Swarm stack is empty
			</p>
			<p style={{color: 'rgba(255,255,255,0.1)', fontSize: 12, margin: 0}}>
				Click <strong style={{color: 'rgba(255,255,255,0.2)'}}>+</strong>{' '}
				to add your first service
			</p>
		</div>
	);
}
