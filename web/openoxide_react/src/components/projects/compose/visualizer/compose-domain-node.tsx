import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {Globe, ExternalLink} from 'lucide-react';

export interface ComposeDomainData extends Record<string, unknown> {
	domain:      string;
	service:     string;
	port?:       string | number;
	https?:      boolean;
}

export function ComposeDomainNode({data, selected}: NodeProps<Node<ComposeDomainData>>) {
	const isHttps = data.https !== false;
	const domainStr = String(data.domain || data.name || data.host || 'domain.com');
	const fullUrl = `${isHttps ? 'https' : 'http'}://${domainStr}`;

	return (
		<div style={{
			width: 220,
			height: 38,
			borderRadius: 10,
			background: '#0a1d1a',
			border: selected
				? '1px solid #10b981'
				: '1px solid rgba(16,185,129,0.35)',
			boxShadow: selected
				? '0 0 0 3px rgba(16,185,129,0.2)'
				: '0 2px 10px rgba(16,185,129,0.08)',
			display: 'flex',
			alignItems: 'center',
			justifyContent: 'space-between',
			padding: '0 10px',
			gap: 8,
			cursor: 'pointer',
		}}>
			{/* Only Target Handle from Service (Left side) */}
			<Handle type="target" position={Position.Left}
				style={{width: 7, height: 7, background: '#10b981', border: '2px solid #0a1d1a', left: -4}} />

			<div style={{
				width: 24, height: 24, borderRadius: 6,
				background: 'rgba(16,185,129,0.15)',
				display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
			}}>
				<Globe size={13} color="#10b981" />
			</div>

			<a
				href={fullUrl}
				target="_blank"
				rel="noreferrer"
				style={{
					flex: 1, minWidth: 0,
					fontSize: 11.5, fontWeight: 600, color: '#ecfdf5',
					textDecoration: 'none',
					whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
					display: 'flex', alignItems: 'center', gap: 4,
				}}
			>
				<span>{domainStr}</span>
				<ExternalLink size={10} color="#6ee7b7" style={{opacity: 0.7, flexShrink: 0}} />
			</a>

			<span style={{
				fontSize: 9.5, fontWeight: 700, fontFamily: 'monospace',
				color: '#10b981', background: 'rgba(16,185,129,0.12)',
				padding: '2px 5px', borderRadius: 4, flexShrink: 0,
			}}>
				:{data.port || 80}
			</span>
		</div>
	);
}
