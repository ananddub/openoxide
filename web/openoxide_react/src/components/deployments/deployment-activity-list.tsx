import {Rocket, Boxes, Database, Clock, AlertCircle, GitBranch} from 'lucide-react';
import type {Deployment} from '#/hooks/deployments/use-deployments';
import {timeAgo} from '#/components/activity/activity-feed';

interface DeploymentActivityListProps {
	deployments: Deployment[];
	onViewLogs: (d: Deployment) => void;
}

/** OpenShip status pill: soft tinted chip, colored text, short label. */
function statusPill(status?: string) {
	const s = (status || '').toUpperCase();
	let cls = 'bg-muted/60 text-muted-foreground';
	let label = s || 'UNKNOWN';
	if (s === 'DONE' || s === 'SUCCESS' || s === 'DEPLOYED' || s === 'HEALTHY') {
		cls = 'bg-emerald-500/10 text-emerald-500';
		label = 'Deployed';
	} else if (s === 'ERROR' || s === 'FAILED' || s === 'CRASHED') {
		cls = 'bg-rose-500/10 text-rose-500';
		label = 'Failed';
	} else if (s === 'CANCELLED' || s === 'CANCELLING') {
		cls = 'bg-muted/60 text-muted-foreground';
		label = 'Canceled';
	} else if (s.includes('BUILD')) {
		cls = 'bg-sky-500/10 text-sky-500';
		label = 'Building';
	} else if (s === 'DEPLOYING' || s === 'STARTING' || s === 'RUNNING') {
		cls = 'bg-sky-500/10 text-sky-500';
		label = 'Deploying';
	} else if (s === 'QUEUED') {
		cls = 'bg-amber-500/10 text-amber-500';
		label = 'Queued';
	}
	return (
		<span
			className={`inline-flex shrink-0 items-center rounded-full px-2 py-0.5 text-[11px] font-medium ${cls}`}>
			{label}
		</span>
	);
}

function kindOf(d: Deployment) {
	if (d.compose_id != null)
		return {icon: Boxes, label: 'Compose'};
	if (d.database_id != null)
		return {icon: Database, label: 'Database'};
	return {icon: Rocket, label: 'Application'};
}

/**
 * OpenShip's deployments list: ONE bordered card, divide-y rows — no
 * Today/Yesterday wrapping. Each row: framework-style icon tile, bold
 * project title + status pill, quiet meta line (actor · time · clock),
 * commit-hash-like id chip on the right. Whole row opens the log stream.
 */
export function DeploymentActivityList({
	deployments,
	onViewLogs,
}: DeploymentActivityListProps) {
	return (
		<div className="divide-y divide-border/50 overflow-hidden rounded-2xl border border-border/50 bg-card">
			{deployments.map(d => {
				const Kind = kindOf(d);
				const Icon = Kind.icon;
				const actor = String((d as any).user_email || '') || 'System';
				const createdAtSec = Number(d.created_at ?? 0);
				return (
					<div
						key={String(d.id)}
						className="group relative flex items-center gap-4 px-4 py-4 transition-colors hover:bg-muted/25">
						<button
							type="button"
							aria-label={`View logs for ${d.title || `deployment #${d.id}`}`}
							onClick={() => onViewLogs(d)}
							className="absolute inset-0 z-0 cursor-pointer"
						/>

						{/* Icon tile */}
						<div className="flex size-9 shrink-0 items-center justify-center overflow-hidden rounded-xl bg-muted/45 transition-colors group-hover:bg-muted/65">
							<Icon className="size-5 text-muted-foreground" />
						</div>

						{/* Main info */}
						<div className="min-w-0 flex-1">
							<div className="mb-0.5 flex flex-wrap items-center gap-2">
								<p className="truncate text-sm font-semibold text-foreground">
									{d.title || `Deployment #${d.id}`}
								</p>
								{statusPill(d.status)}
							</div>
							<div className="mt-0.5 flex items-center gap-2">
								<p className="max-w-[320px] truncate text-xs text-muted-foreground">
									{d.description || `${actor} · ${Kind.label}`}
								</p>
								<span className="text-muted-foreground/40">·</span>
								<span className="shrink-0 text-xs text-muted-foreground">
									{timeAgo(createdAtSec)}
								</span>
								<span className="text-muted-foreground/40">·</span>
								<span className="flex shrink-0 items-center gap-1 text-xs text-muted-foreground">
									<Clock className="size-3" />
									{new Date(createdAtSec * 1000).toLocaleTimeString(
										[],
										{hour: '2-digit', minute: '2-digit'},
									)}
								</span>
							</div>
							{d.error_message && (
								<p className="mt-1 flex max-w-md items-center gap-1.5 truncate text-xs font-medium text-rose-500">
									<AlertCircle className="size-3 shrink-0" />
									<span className="truncate">{d.error_message}</span>
								</p>
							)}
						</div>

						{/* Right: id chip (commit-hash style) */}
						<div className="relative z-10 flex shrink-0 items-center gap-2">
							<span className="hidden items-center gap-1.5 rounded-lg px-2.5 py-1.5 font-mono text-xs text-muted-foreground transition-colors group-hover:bg-muted/50 group-hover:text-foreground sm:inline-flex">
								<GitBranch className="size-3 opacity-60" />#{d.id}
							</span>
						</div>
					</div>
				);
			})}
		</div>
	);
}
