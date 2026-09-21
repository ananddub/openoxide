import {Link} from '@tanstack/react-router';
import {ArrowRight, ArrowUpRight} from 'lucide-react';
import {ActivitySummary} from '#/components/activity/activity-feed';
import {
	MeterBar,
	DeployBars,
	HealthRow,
	DeltaTag,
} from '#/components/home/overview-widgets';

interface OverviewRightColumnProps {
	runningPct: number;
	status: {running: number; idle: number; error: number};
	isLoading: boolean;
	deployStats: {value: string | number; delta?: string};
	deployDays: {label: string; count: number}[];
}

const SHADOW =
	'shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)]';

/** The sticky overview sidebar: service health, deployments and activity. */
export function OverviewRightColumn({
	runningPct,
	status,
	isLoading,
	deployStats,
	deployDays,
}: OverviewRightColumnProps) {
	return (
		<div className="space-y-6 lg:sticky lg:top-6 lg:self-start">
			<div className={`rounded-lg border bg-card p-5 ${SHADOW}`}>
				<div className="mb-4 flex items-center justify-between">
					<h3 className="text-sm font-semibold text-foreground">
						Service health
					</h3>
					<span className="text-xs text-muted-foreground">
						{runningPct}% running
					</span>
				</div>
				<MeterBar
					segments={[
						{
							label: 'Running',
							value: status.running,
							className: 'bg-emerald-500',
						},
						{
							label: 'Errors',
							value: status.error,
							className: 'bg-red-500',
						},
						{
							label: 'Idle',
							value: status.idle,
							className: 'bg-foreground/25',
						},
					]}
				/>
				<div className="mt-4 space-y-2.5">
					<HealthRow
						label="Running"
						count={status.running}
						dot="bg-emerald-500"
					/>
					<HealthRow
						label="Idle"
						count={status.idle}
						dot="bg-foreground/25"
					/>
					<HealthRow
						label="Errored"
						count={status.error}
						dot="bg-red-500"
					/>
				</div>
			</div>

			<div className={`rounded-lg border bg-card p-5 ${SHADOW}`}>
				<div className="mb-4 flex items-center justify-between">
					<h3 className="text-sm font-semibold text-foreground">
						Deployments
					</h3>
					<Link
						to="/Deployments"
						className="flex items-center gap-1 text-xs text-muted-foreground transition-colors hover:text-foreground">
						History
						<ArrowUpRight className="size-3" />
					</Link>
				</div>
				<DeployBars days={deployDays} />
				<div className="mt-3 flex items-baseline justify-between">
					<span className="text-2xl font-semibold tracking-[-0.02em] text-foreground">
						{isLoading ? '…' : deployStats.value}
					</span>
					<DeltaTag delta={deployStats.delta} />
				</div>
				<p className="mt-1 text-xs text-muted-foreground">Last 14 days</p>
			</div>

			<div className={`rounded-lg border bg-card p-5 ${SHADOW}`}>
				<div className="mb-4 flex items-center justify-between">
					<h3 className="text-sm font-semibold text-foreground">
						Recent activity
					</h3>
					<Link
						to="/settings"
						search={{tab: 'audit'}}
						className="flex items-center gap-1 text-xs text-muted-foreground transition-colors hover:text-foreground">
						View all
						<ArrowRight className="size-3" />
					</Link>
				</div>
				<ActivitySummary count={6} />
			</div>
		</div>
	);
}
