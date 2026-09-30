import {ChevronDown, Clock, Zap} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	ListCard,
	ListSkeleton,
	ListEmpty,
} from '#/components/settings/list-card';
import {formatRelative, getStatusTone} from './build-status';
import {DeploymentInlineLogs} from './deployment-inline-logs';

export interface DeploymentsListProps {
	deployments: any[];
	isLoading?: boolean;
	emptyLabel?: string;
	emptyDescription?: string;
	/** Prefix for the synthetic title of rows without one. */
	titlePrefix?: string;
	/** Status shown when a row carries neither status nor state. */
	defaultStatus?: string;
	/** Row whose inline log panel is open. */
	expandedId?: number | null;
	onToggleExpand?: (id: number) => void;
}

export function DeploymentsList({
	deployments,
	isLoading,
	emptyLabel = 'No deployments yet',
	emptyDescription = 'Trigger a deploy to start the history.',
	titlePrefix = 'Deployment',
	defaultStatus = 'PENDING',
	expandedId = null,
	onToggleExpand,
}: DeploymentsListProps) {
	const safe = Array.isArray(deployments) ? deployments : [];

	if (isLoading && safe.length === 0) return <ListSkeleton />;
	if (safe.length === 0)
		return (
			<ListEmpty
				icon={Zap}
				title={emptyLabel}
				description={emptyDescription}
			/>
		);

	return (
		<ListCard className="divide-border/60">
			{safe.map((e: any) => {
				const tone = getStatusTone(e);
				const isOpen = Number(e.id) === Number(expandedId);
				return (
					<div key={e.id}>
						<button
							type="button"
							onClick={() => onToggleExpand?.(Number(e.id))}
							className="group flex w-full cursor-pointer items-center justify-between gap-4 px-4 py-3.5 text-left transition-colors hover:bg-accent/30">
							<div className="flex min-w-0 items-center gap-3">
								<span
									aria-hidden
									className={cn(
										'size-2 shrink-0 rounded-full',
										tone.dot,
										tone.pulse && 'animate-pulse',
									)}
								/>
								<div className="min-w-0">
									<span className="block truncate text-sm font-medium text-foreground">
										{e.title || `${titlePrefix} #${e.id}`}
									</span>
									{(e.description || e.message) && (
										<span className="block truncate text-xs text-muted-foreground">
											{e.description || e.message}
										</span>
									)}
								</div>
							</div>

							<div className="flex shrink-0 items-center gap-3">
								<span
									className={cn(
										'text-xs font-medium lowercase',
										tone.text,
									)}>
									{String(
										e.status || e.state || defaultStatus,
									).toLowerCase()}
								</span>
								<span
									className="flex items-center gap-1 font-mono text-xs text-muted-foreground tabular-nums"
									title={
										e.created_at
											? new Date(
													Number(e.created_at) < 1e11
														? Number(e.created_at) * 1000
														: Number(e.created_at),
												).toLocaleString()
											: undefined
									}>
									<Clock className="size-3" />
									{formatRelative(e.created_at)}
								</span>
								<ChevronDown
									className={cn(
										'size-3.5 text-muted-foreground transition-transform duration-200',
										isOpen && 'rotate-180',
									)}
								/>
							</div>
						</button>
						{isOpen && <DeploymentInlineLogs deployment={e} />}
					</div>
				);
			})}
		</ListCard>
	);
}
