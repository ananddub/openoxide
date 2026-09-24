import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Activity,
	Check,
	Copy,
	Hash,
	LogOut,
	Network,
	ShieldAlert,
	UserCog,
} from 'lucide-react';
import {toast} from 'sonner';

import type {SwarmInfo} from '#/types/api-helpers';

interface SwarmInfoCardProps {
	info?: SwarmInfo | null;
	isLoading: boolean;
	onLeaveSwarm: () => void;
}

async function copyText(value: string, label: string) {
	try {
		await navigator.clipboard.writeText(value);
		toast.success(`${label} copied`);
	} catch {
		toast.error('Could not access the clipboard');
	}
}

export function SwarmInfoCard({
	info,
	isLoading,
	onLeaveSwarm,
}: SwarmInfoCardProps) {
	const [copiedId, setCopiedId] = useState(false);

	if (isLoading) {
		/* Card-shaped placeholder — same footprint as the real rail card. */
		return (
			<div className="overflow-hidden rounded-xl border bg-card">
				<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
					<Skeleton className="size-9 shrink-0 rounded-lg" />
					<div className="flex-1 space-y-2">
						<Skeleton className="h-3.5 w-32" />
						<Skeleton className="h-3 w-20" />
					</div>
					<Skeleton className="h-5 w-16 rounded-full" />
				</div>
				<div className="divide-y divide-border/40 px-5">
					{[1, 2, 3, 4].map(i => (
						<div key={i} className="flex items-center gap-3 py-3">
							<Skeleton className="size-8 shrink-0 rounded-lg" />
							<div className="flex-1 space-y-1.5">
								<Skeleton className="h-3 w-24" />
								<Skeleton className="h-3.5 w-16" />
							</div>
						</div>
					))}
				</div>
			</div>
		);
	}

	const isActive =
		(info?.local_node_state || '').toLowerCase() === 'active';

	if (!isActive) {
		return (
			<div className="flex flex-col items-center justify-center gap-4 rounded-xl border bg-card px-6 py-16 text-center">
				<div className="flex size-12 items-center justify-center rounded-full bg-amber-500/10 text-amber-500">
					<ShieldAlert className="size-5" />
				</div>
				<div>
					<p className="text-sm font-medium text-foreground">
						Docker Swarm Inactive
					</p>
					<p className="mx-auto mt-1 max-w-sm text-xs text-muted-foreground">
						This host engine is running in standalone mode. Initialize
						Docker Swarm or join an existing manager node.
					</p>
				</div>
			</div>
		);
	}

	const nodeId = info?.node_id || '';

	return (
		<div className="overflow-hidden rounded-xl border border-border bg-card">
			{/* Header — identity + state */}
			<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
					<Network className="size-[18px] text-muted-foreground" />
				</div>
				<div className="min-w-0 flex-1">
					<h3 className="truncate text-[15px] font-semibold text-foreground">
						Cluster Overview
					</h3>
					<p className="truncate text-xs text-muted-foreground">
						{info?.control_available ? 'Manager' : 'Worker'} engine
					</p>
				</div>
				<span className="inline-flex shrink-0 items-center gap-1.5 rounded-full bg-emerald-500/10 px-2.5 py-1 text-[11px] font-medium text-emerald-500">
					<span className="size-1.5 rounded-full bg-emerald-500" />
					{info?.local_node_state || 'active'}
				</span>
			</div>

			{/* Stat rows */}
			<div className="divide-y divide-border/40 px-5">
				<Stat
					icon={Activity}
					label="Swarm Nodes"
					value={String(info?.nodes ?? 0)}
					sub="total in cluster"
				/>
				<Stat
					icon={UserCog}
					label="Managers"
					value={String(info?.managers ?? 0)}
					sub="quorum members"
				/>
				<Stat
					icon={Network}
					label="Node Address"
					value={info?.node_addr || '127.0.0.1'}
					mono
				/>
				<div className="flex items-center gap-3 py-3">
					<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
						<Hash className="size-4 text-muted-foreground" />
					</div>
					<div className="min-w-0 flex-1">
						<p className="text-xs text-muted-foreground">Node ID</p>
						<p className="mt-0.5 truncate font-mono text-sm font-semibold text-foreground">
							{nodeId ? nodeId.slice(0, 12) : '—'}
						</p>
					</div>
					{nodeId && (
						<button
							type="button"
							onClick={() => {
								void copyText(nodeId, 'Node ID');
								setCopiedId(true);
								setTimeout(() => setCopiedId(false), 1500);
							}}
							title="Copy full node ID"
							className="flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
							{copiedId ? (
								<Check className="size-4 text-emerald-500" />
							) : (
								<Copy className="size-4" />
							)}
						</button>
					)}
				</div>
			</div>

			{/* Footer — destructive action */}
			<div className="border-t border-border/50 px-5 py-3">
				<Button
					variant="ghost"
					size="sm"
					onClick={onLeaveSwarm}
					title="Leave Swarm Cluster"
					className="h-8 w-full cursor-pointer gap-1.5 text-destructive hover:bg-destructive/10 hover:text-destructive">
					<LogOut className="size-3.5" />
					Leave Swarm
				</Button>
			</div>
		</div>
	);
}

function Stat({
	icon: Icon,
	label,
	value,
	sub,
	mono = false,
}: {
	icon: React.ComponentType<{className?: string}>;
	label: string;
	value: string;
	sub?: string;
	mono?: boolean;
}) {
	return (
		<div className="flex items-center gap-3 py-3">
			<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05]">
				<Icon className="size-4 text-muted-foreground" />
			</div>
			<div className="min-w-0">
				<p className="text-xs text-muted-foreground">{label}</p>
				<p
					className={`mt-0.5 truncate text-sm font-semibold text-foreground ${mono ? 'font-mono' : ''}`}>
					{value}
				</p>
				{sub && (
					<p className="truncate text-[11px] text-muted-foreground/70">
						{sub}
					</p>
				)}
			</div>
		</div>
	);
}
