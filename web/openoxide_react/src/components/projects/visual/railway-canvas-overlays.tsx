import {MousePointer2, MoveRight} from 'lucide-react';

// ─── Select / Arrow pill toolbar ──────────────────────────────────────────────

const TOOLS = [
	{
		icon: <MousePointer2 size={13} />,
		label: 'Select  V',
		key: 'select' as const,
	},
	{
		icon: <MoveRight size={13} />,
		label: 'Arrow  A',
		key: 'arrow' as const,
	},
];

export function ModeToolbar({
	mode,
	onMode,
}: {
	mode: 'select' | 'arrow';
	onMode: (m: 'select' | 'arrow') => void;
}) {
	return (
		<div
			style={{
				position: 'absolute',
				top: 12,
				left: '50%',
				transform: 'translateX(-50%)',
				zIndex: 20,
				display: 'flex',
				alignItems: 'center',
				gap: 2,
				padding: '3px 5px',
				borderRadius: 99,
				background: '#161b22',
				border: '1px solid rgba(255,255,255,0.08)',
				boxShadow: '0 1px 8px rgba(0,0,0,0.3)',
			}}>
			{TOOLS.map(t => (
				<button
					key={t.key}
					type="button"
					title={t.label}
					onClick={() => onMode(t.key)}
					style={{
						display: 'flex',
						alignItems: 'center',
						gap: 5,
						padding: '4px 10px',
						borderRadius: 99,
						border: 'none',
						cursor: 'pointer',
						fontSize: 11.5,
						fontWeight: 500,
						background:
							mode === t.key
								? t.key === 'arrow'
									? 'rgba(88,166,255,0.15)'
									: 'rgba(255,255,255,0.08)'
								: 'transparent',
						color:
							mode === t.key
								? t.key === 'arrow'
									? '#58a6ff'
									: '#e6edf3'
								: '#6e7681',
						transition: 'all .12s',
					}}>
					{t.icon}
					{mode === t.key ? t.label.split('  ')[0] : ''}
				</button>
			))}
		</div>
	);
}

export function ArrowHint() {
	return (
		<div
			style={{
				position: 'absolute',
				bottom: 12,
				left: '50%',
				transform: 'translateX(-50%)',
				zIndex: 20,
				background: '#161b22',
				border: '1px solid rgba(88,166,255,0.3)',
				borderRadius: 99,
				padding: '4px 14px',
				fontSize: 11,
				color: '#58a6ff',
				fontWeight: 500,
				display: 'flex',
				alignItems: 'center',
				gap: 6,
				boxShadow: '0 1px 8px rgba(0,0,0,0.3)',
				pointerEvents: 'none',
			}}>
			<MoveRight size={12} />
			Drag to draw connection — press V to exit
		</div>
	);
}

// ─── Right-click "Add Service" menu ───────────────────────────────────────────

export function RailwayAddMenu({
	x,
	y,
	onPick,
	items,
}: {
	x: number;
	y: number;
	onPick: (fn?: () => void) => void;
	items: {label: string; fn?: () => void}[];
}) {
	return (
		<div
			onClick={e => e.stopPropagation()}
			style={{
				position: 'absolute',
				top: y,
				left: x,
				zIndex: 50,
				background: '#161b22',
				border: '1px solid rgba(255,255,255,0.1)',
				borderRadius: 10,
				padding: 4,
				minWidth: 160,
				boxShadow: '0 8px 24px rgba(0,0,0,0.5)',
			}}>
			<p
				style={{
					margin: '2px 10px 6px',
					fontSize: 10,
					fontWeight: 600,
					color: '#6e7681',
					textTransform: 'uppercase',
					letterSpacing: '0.06em',
				}}>
				Add Service
			</p>
			{items.map(({label, fn}) => (
				<button
					key={label}
					type="button"
					onClick={() => onPick(fn)}
					style={{
						display: 'block',
						width: '100%',
						textAlign: 'left',
						padding: '6px 10px',
						border: 'none',
						borderRadius: 7,
						background: 'transparent',
						cursor: 'pointer',
						fontSize: 12.5,
						color: '#e6edf3',
						transition: 'background .1s',
					}}
					onMouseEnter={e =>
						(e.currentTarget.style.background = 'rgba(255,255,255,0.06)')
					}
					onMouseLeave={e =>
						(e.currentTarget.style.background = 'transparent')
					}>
					{label}
				</button>
			))}
		</div>
	);
}
