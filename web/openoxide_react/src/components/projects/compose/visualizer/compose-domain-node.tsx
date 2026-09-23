import {type NodeProps, type Node, Handle, Position} from '@xyflow/react';
import {Globe, ExternalLink} from 'lucide-react';
import {cn} from '#/api/utils';

export interface ComposeDomainData extends Record<string, unknown> {
	domain: string;
	service: string;
	port?: string | number;
	https?: boolean;
}

export function ComposeDomainNode({
	data,
	selected,
}: NodeProps<Node<ComposeDomainData>>) {
	const isHttps = data.https !== false;
	const domainStr = String(
		data.domain || data.name || data.host || 'domain.com',
	);
	const fullUrl = `${isHttps ? 'https' : 'http'}://${domainStr}`;

	return (
		<div
			className={cn(
				'flex h-[38px] w-[220px] cursor-pointer items-center justify-between gap-2 rounded-xl border border-emerald-500/45 bg-card px-2.5 shadow-[0_1px_2px_rgb(0_0_0/0.05),0_4px_12px_rgb(0_0_0/0.08)] transition-[border-color,box-shadow] duration-100 dark:shadow-[0_2px_12px_rgb(0_0_0/0.4)]',
				selected
					? 'border-emerald-500 shadow-[0_0_0_3px_rgb(16_185_129/0.2)]'
					: 'border-emerald-500/30',
			)}>
			{/* Only Target Handle from Service (Left side) */}
			<Handle
				type="target"
				position={Position.Left}
				style={{
					width: 7,
					height: 7,
					background: '#10b981',
					border: '2px solid var(--card)',
					left: -4,
				}}
			/>

			<div className="flex size-6 shrink-0 items-center justify-center rounded-[6px] bg-emerald-500 shadow-sm">
				<Globe size={13} className="text-white" />
			</div>

			<a
				href={fullUrl}
				target="_blank"
				rel="noreferrer"
				className="flex min-w-0 flex-1 items-center gap-1 truncate text-[11.5px] font-semibold text-foreground no-underline">
				<span className="truncate">{domainStr}</span>
				<ExternalLink
					size={10}
					className="shrink-0 text-muted-foreground opacity-70"
				/>
			</a>

			<span className="shrink-0 rounded bg-emerald-500/15 px-[5px] py-[2px] font-mono text-[9.5px] font-bold text-emerald-600 dark:text-emerald-400">
				:{data.port || 80}
			</span>
		</div>
	);
}
