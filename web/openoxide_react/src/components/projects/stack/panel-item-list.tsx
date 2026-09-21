import {useState} from 'react';
import {Plus, Minus} from 'lucide-react';
import {
	SectionLabel,
	iconBtnStyle,
	dashedInputStyle,
	addBtnStyle,
} from './panel-primitives';

/** Read-only string list with dashed add-input — shared by volumes & ports. */
export function PanelItemList({
	label,
	items,
	placeholder,
	onAdd,
	onRemove,
}: {
	label: string;
	items: string[];
	placeholder: string;
	onAdd: (value: string) => void;
	onRemove: (index: number) => void;
}) {
	const [next, setNext] = useState('');

	const add = () => {
		if (!next.trim()) return;
		onAdd(next.trim());
		setNext('');
	};

	return (
		<div>
			<SectionLabel>{label}</SectionLabel>
			{items.map((item, i) => (
				<div key={i} style={{display: 'flex', gap: 5, marginBottom: 4}}>
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
						{item}
					</span>
					<button
						type="button"
						onClick={() => onRemove(i)}
						style={iconBtnStyle('#f85149', 'rgba(248,81,73,0.25)')}>
						<Minus size={9} />
					</button>
				</div>
			))}
			<div style={{display: 'flex', gap: 5, marginTop: 4}}>
				<input
					value={next}
					onChange={e => setNext(e.target.value)}
					placeholder={placeholder}
					onKeyDown={e => e.key === 'Enter' && add()}
					style={dashedInputStyle}
				/>
				<button type="button" onClick={add} style={addBtnStyle}>
					<Plus size={11} />
				</button>
			</div>
		</div>
	);
}
