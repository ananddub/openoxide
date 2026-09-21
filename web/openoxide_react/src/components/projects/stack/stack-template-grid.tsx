import {TEMPLATES, type Template} from './stack-templates';

/** 2-column selectable grid of service template cards. */
export function StackTemplateGrid({
	selected,
	onSelect,
}: {
	selected: Template | null;
	onSelect: (t: Template) => void;
}) {
	return (
		<div
			style={{
				padding: 14,
				display: 'grid',
				gridTemplateColumns: '1fr 1fr',
				gap: 8,
			}}>
			{TEMPLATES.map(t => (
				<TemplateButton
					key={t.key}
					t={t}
					active={selected?.key === t.key}
					onSelect={onSelect}
				/>
			))}
		</div>
	);
}

function TemplateButton({
	t,
	active,
	onSelect,
}: {
	t: Template;
	active: boolean;
	onSelect: (t: Template) => void;
}) {
	return (
		<button
			type="button"
			onClick={() => onSelect(t)}
			style={{
				display: 'flex',
				alignItems: 'center',
				gap: 10,
				padding: '10px 12px',
				borderRadius: 9,
				border: 'none',
				cursor: 'pointer',
				textAlign: 'left',
				background: active
					? 'rgba(88,166,255,0.1)'
					: 'rgba(255,255,255,0.03)',
				outline: active
					? '1px solid rgba(88,166,255,0.4)'
					: '1px solid rgba(255,255,255,0.06)',
				transition: 'all .1s',
			}}
			onMouseEnter={e => {
				if (!active)
					e.currentTarget.style.background = 'rgba(255,255,255,0.06)';
			}}
			onMouseLeave={e => {
				if (!active)
					e.currentTarget.style.background = 'rgba(255,255,255,0.03)';
			}}>
			<div
				style={{
					width: 30,
					height: 30,
					borderRadius: 7,
					flexShrink: 0,
					background: `${t.color}18`,
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
				}}>
				<svg
					viewBox="0 0 24 24"
					style={{width: 15, height: 15, fill: t.color}}>
					<path d={t.icon} />
				</svg>
			</div>
			<div>
				<p
					style={{
						margin: 0,
						fontSize: 12.5,
						fontWeight: 600,
						color: '#e6edf3',
					}}>
					{t.label}
				</p>
				<p style={{margin: 0, fontSize: 10.5, color: '#6e7681'}}>
					{t.sub}
				</p>
			</div>
		</button>
	);
}
