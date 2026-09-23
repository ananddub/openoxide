import {useState} from 'react';
import {
	ChevronDown,
	Rocket,
	Boxes,
	Database,
	AlertCircle,
} from 'lucide-react';
import type {Deployment} from '#/hooks/deployments/use-deployments';
import {timeAgo} from '#/components/activity/activity-feed';
import {cn} from '#/api/utils';
import {
	getStatusTone,
} from '#/components/projects/detail/deployments/build-status';
import {DeploymentInlineLogs} from '#/components/projects/detail/deployments/deployment-inline-logs';
import {useAutoExpandBuildLogs} from '#/components/projects/detail/deployments/use-auto-expand-build-logs';

interface DeploymentActivityListProps {
	deployments: Deployment[];
}

function kindOf(d: Deployment) {
	if (d.compose_id != null) return {icon: Boxes, label: 'Compose'};
	if (d.database_id != null) return {icon: Database, label: 'Database'};
	return {icon: Rocket, label: 'Application'};
}

/**
 * Global deployments list: ONE bordered card, divide-y rows. Each row:
 * kind icon tile, title + quiet meta line, status dot/text, relative
 * time, chevron. Clicking a row opens the streaming log panel inline
 * (drag-resizable) — no modal.
 */
export function DeploymentActivityList({
	deployments,
}: DeploymentActivityListProps) {
	const [expandedId, setExpandedId] = useState<number | null>(null);
	useAutoExpandBuildLogs(deployments as any[], setExpandedId);

	return (
		<div className="divide-y divide-border/50 overflow-hidden rounded-xl border border-border/50 bg-card">
			{deployments.map(d => {
				const Kind = kindOf(d);
				const Icon = Kind.icon;
				const tone = getStatusTone(d);
				const actor = String((d as any).user_email || '') || 'System';
				const createdAtSec = Number(d.created_at ?? 0);
				const id = Number(d.id);
				const isOpen = expandedId === id;
				return (
					<div key={String(d.id)}>
						<button
							type="button"
							onClick={() =>
								setExpandedId(prev =>
									prev === id ? null : id,
								)
							}
							className="flex w-full cursor-pointer items-center gap-4 px-4 py-3.5 text-left transition-colors hover:bg-muted/20">
							<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted/45">
								<Icon className="size-[18px] text-muted-foreground" />
							</div>

							<div className="min-w-0 flex-1">
								<p className="truncate text-[13px] font-medium text-foreground">
									{d.title || `Deployment #${d.id}`}
								</p>
								<p className="mt-0.5 truncate text-xs text-muted-foreground">
									{d.description ||
										`${actor} · ${Kind.label}`}
								</p>
								{d.error_message && (
									<p className="mt-1 flex max-w-md items-center gap-1.5 truncate text-xs font-medium text-rose-500">
										<AlertCircle className="size-3 shrink-0" />
										<span className="truncate">
											{d.error_message}
										</span>
									</p>
								)}
							</div>

							<div className="flex shrink-0 items-center gap-3">
								<span
									aria-hidden
									className={cn(
										'size-2 shrink-0 rounded-full',
										tone.dot,
										tone.pulse && 'animate-pulse',
									)}
								/>
								<span
									className={cn(
										'hidden text-xs font-medium lowercase sm:block',
										tone.text,
									)}>
									{String(d.status || 'unknown').toLowerCase()}
								</span>
								<span className="hidden text-xs text-muted-foreground tabular-nums md:block">
									{timeAgo(createdAtSec)}
								</span>
								<ChevronDown
									className={cn(
										'size-3.5 text-muted-foreground transition-transform duration-200',
										isOpen && 'rotate-180',
									)}
								/>
							</div>
						</button>
						{isOpen && <DeploymentInlineLogs deployment={d} />}
					</div>
				);
			})}
		</div>
	);
}
