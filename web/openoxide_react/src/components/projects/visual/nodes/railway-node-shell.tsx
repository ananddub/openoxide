import {useState, type ReactNode, type CSSProperties} from 'react';
import {Handle, Position} from '@xyflow/react';

const HANDLE_STYLE: CSSProperties = {
	width: 10,
	height: 10,
	borderRadius: '50%',
	border: '2px solid #30363d',
	background: '#0d1117',
	transition: 'border-color .15s, box-shadow .15s',
};

export interface NodeShellProps {
	selected: boolean;
	running: boolean;
	iconBg: string;
	selectedBorder: string;
	selectedShadow: string;
	icon: ReactNode;
	name: string;
	subtitle: ReactNode;
	subtitleStyle?: CSSProperties;
	handleAccent?: string;
	onClick: () => void;
}

/** Shared hover/status/icon/text card chrome used by all Railway canvas nodes. */
export function NodeShell({
	selected,
	running,
	iconBg,
	selectedBorder,
	selectedShadow,
	icon,
	name,
	subtitle,
	subtitleStyle,
	handleAccent,
	onClick,
}: NodeShellProps) {
	const [hov, setHov] = useState(false);
	const borderColor = selected
		? selectedBorder
		: hov
			? '1px solid rgba(255,255,255,0.16)'
			: '1px solid rgba(255,255,255,0.08)';

	return (
		<div
			onClick={onClick}
			onMouseEnter={() => setHov(true)}
			onMouseLeave={() => setHov(false)}
			style={{
				display: 'flex',
				alignItems: 'center',
				gap: 10,
				width: 185,
				padding: '10px 12px',
				background: '#161b22',
				border: borderColor,
				borderRadius: 10,
				cursor: 'pointer',
				userSelect: 'none',
				boxShadow: selected ? selectedShadow : 'none',
				transition: 'border-color .12s, box-shadow .12s',
				position: 'relative',
			}}>
			{/* Status dot */}
			<span
				style={{
					position: 'absolute',
					top: 7,
					right: 8,
					width: 6,
					height: 6,
					borderRadius: '50%',
					background: running ? '#3fb950' : '#6e7681',
					boxShadow: running ? '0 0 6px #3fb950' : 'none',
				}}
			/>

			{/* Icon */}
			<div
				style={{
					width: 28,
					height: 28,
					borderRadius: 7,
					background: iconBg,
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center',
					flexShrink: 0,
				}}>
				{icon}
			</div>

			{/* Name + subtitle */}
			<div style={{flex: 1, minWidth: 0}}>
				<p
					style={{
						margin: 0,
						fontSize: 12.5,
						fontWeight: 600,
						color: '#e6edf3',
						lineHeight: 1.2,
						whiteSpace: 'nowrap',
						overflow: 'hidden',
						textOverflow: 'ellipsis',
						paddingRight: 14,
					}}>
					{name}
				</p>
				<p
					style={{
						margin: 0,
						fontSize: 10,
						color: '#6e7681',
						marginTop: 2,
						...subtitleStyle,
					}}>
					{subtitle}
				</p>
			</div>

			<Handle
				type="target"
				position={Position.Left}
				style={{...HANDLE_STYLE, left: -6, borderColor: handleAccent}}
			/>
			<Handle
				type="source"
				position={Position.Right}
				style={{...HANDLE_STYLE, right: -6, borderColor: handleAccent}}
			/>
		</div>
	);
}
