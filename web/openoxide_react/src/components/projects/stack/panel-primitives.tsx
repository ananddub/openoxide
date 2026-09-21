import {useState, useEffect, useRef} from 'react';

export function SectionLabel({children}: {children: React.ReactNode}) {
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
export function LiveInput({
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

export const iconBtnStyle = (
	color: string,
	border: string,
	size = 22,
): React.CSSProperties => ({
	width: size,
	height: size,
	borderRadius: 5,
	background: 'transparent',
	border: `1px solid ${border}`,
	cursor: 'pointer',
	color,
	display: 'flex',
	alignItems: 'center',
	justifyContent: 'center',
	flexShrink: 0,
});

export const dashedInputStyle: React.CSSProperties = {
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
};

export const addBtnStyle: React.CSSProperties = {
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
};
