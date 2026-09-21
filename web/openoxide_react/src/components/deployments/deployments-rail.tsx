import {
	Activity,
	CheckCircle2,
	Layers,
	Loader2,
	XCircle,
} from 'lucide-react';
import {cn} from '#/api/utils';
import {TABS, type DeploymentsTabId} from './deployments-tabs';
import {DeploymentsQuickTip} from './deployments-quick-tip';

type Stats = {
	total: number;
	success: number;
	failed: number;
	inProgress: number;
};

/** Sticky right rail: section nav, Activity Overview, quick tip. */
export function DeploymentsRail({
	active,
	onSelect,
	historyCount,
	queueCount,
	stats,
	isLoading,
}: {
	active: DeploymentsTabId;
	onSelect: (id: DeploymentsTabId) => void;
	historyCount: number;
	queueCount: number;
	stats: Stats;
	isLoading: boolean;
}) {
	const rows: {
		key: string;
		label: string;
		icon: typeof Layers;
		box: string;
		cls: string;
		value: number;
		pulse?: boolean;
		show: boolean;
	}[] = [
		{
			key: 'total',
			label: 'Total',
			icon: Layers,
			box: 'bg-primary/10',
			cls: 'text-primary',
			value: stats.total,
			show: true,
		},
		{
			key: 'success',
			label: 'Successful',
			icon: CheckCircle2,
			box: 'bg-emerald-500/10',
			cls: 'text-emerald-500',
			value: stats.success,
			show: true,
		},
		{
			key: 'failed',
			label: 'Failed',
			icon: XCircle,
			box: 'bg-rose-500/10',
			cls: 'text-rose-500',
			value: stats.failed,
			show: true,
		},
		{
			key: 'inProgress',
			label: 'In progress',
			icon: Loader2,
			box: 'bg-amber-500/10',
			cls: 'animate-spin text-amber-500',
			value: stats.inProgress,
			show: stats.inProgress > 0,
		},
	];

	return (
		<div className="hidden space-y-4 lg:sticky lg:top-6 lg:block lg:self-start">
			{/* Section nav */}
			<div className="rounded-xl bg-card p-3">
				<div className="space-y-1">
					{TABS.map(t => {
						const isActive = t.id === active;
						const count = t.id === 'history' ? historyCount : queueCount;
						return (
							<button
								key={t.id}
								type="button"
								onClick={() => onSelect(t.id)}
								className={cn(
									'flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-[14px] font-medium transition-colors',
									isActive
										? 'bg-foreground/[0.07] text-foreground'
										: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
								)}>
								<t.icon
									className={cn(
										'size-[17px] shrink-0',
										t.id === 'queue' &&
											queueCount > 0 &&
											'animate-spin text-amber-500',
									)}
									strokeWidth={1.7}
								/>
								<span className="flex-1 text-start">{t.label}</span>
								<span
									className={cn(
										'rounded-md px-1.5 py-0.5 font-mono text-[10px]',
										t.id === 'queue' && queueCount > 0
											? 'animate-pulse bg-amber-500 font-bold text-black'
											: 'bg-muted/60 text-muted-foreground',
									)}>
									{count}
								</span>
							</button>
						);
					})}
				</div>
			</div>

			{/* Activity Overview */}
			<div className="rounded-xl border border-border/50 bg-card p-5">
				<div className="mb-4 flex items-center gap-2">
					<Activity className="size-4 text-muted-foreground" />
					<h3 className="text-sm font-semibold text-foreground">
						Activity Overview
					</h3>
				</div>

				<div className="space-y-3">
					{rows
						.filter(r => r.show)
						.map(r => (
							<div
								key={r.key}
								className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div
										className={cn(
											'flex size-8 items-center justify-center rounded-lg',
											r.box,
										)}>
										<r.icon className={cn('size-4', r.cls)} />
									</div>
									<span className="text-sm text-muted-foreground">
										{r.label}
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading && r.key !== 'inProgress' ? '–' : r.value}
								</span>
							</div>
						))}
				</div>

				{/* Fleet-health proportion bar */}
				{!isLoading && stats.total > 0 && (
					<div className="mt-4 flex h-1.5 overflow-hidden rounded-full bg-muted/40">
						{stats.success > 0 && (
							<div
								className="bg-emerald-500"
								style={{width: `${(stats.success / stats.total) * 100}%`}}
							/>
						)}
						{stats.failed > 0 && (
							<div
								className="bg-rose-500"
								style={{width: `${(stats.failed / stats.total) * 100}%`}}
							/>
						)}
						{stats.inProgress > 0 && (
							<div
								className="bg-amber-500"
								style={{
									width: `${(stats.inProgress / stats.total) * 100}%`,
								}}
							/>
						)}
					</div>
				)}
			</div>

			{/* Quick tip */}
			<DeploymentsQuickTip total={stats.total} />
		</div>
	);
}
