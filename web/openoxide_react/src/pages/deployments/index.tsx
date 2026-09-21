import {createFileRoute, useNavigate, useSearch} from '@tanstack/react-router';
import {
	RefreshCw,
	Activity,
	CheckCircle2,
	Layers,
	Terminal,
	XCircle,
	Clock,
	Loader2,
	Zap,
	ArrowRight,
	Trash2,
} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import {DeploymentActivityList} from '#/components/deployments/deployment-activity-list';
import {useEffect, useMemo, useState} from 'react';
import {useDeployments} from '#/hooks/deployments/use-deployments';
import {DeploymentsFilters} from '#/components/deployments/deployments-filters';
import {DeploymentLogsDialog} from '#/components/deployments/deployment-logs-dialog';
import {DeploymentErrorDialog} from '#/components/deployments/deployment-error-dialog';
import {EmptyState} from '#/components/activity/empty-state';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {cn} from '#/api/utils';

type DeploymentsTabId = 'history' | 'queue';

const TABS: {id: DeploymentsTabId; label: string; icon: typeof Layers}[] = [
	{id: 'history', label: 'Logs & History', icon: Layers},
	{id: 'queue', label: 'Active Queue', icon: Activity},
];

export const Route = createFileRoute('/_app/Deployments')({
	component: DeploymentsPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {id?: string; tab?: DeploymentsTabId} => ({
		id: typeof search.id === 'string' && search.id ? search.id : undefined,
		tab: search.tab === 'queue' ? 'queue' : undefined,
	}),
});

function DeploymentsPage() {
	const {
		isLoading,
		refreshing,
		handleRefresh,
		searchQuery,
		setSearchQuery,
		statusFilter,
		setStatusFilter,
		typeFilter,
		setTypeFilter,
		sortBy,
		sortDir,
		setSortBy,
		setSortDir,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		isLogsLoading,
		copied,
		handleCopyLogs,
		errorDetailDeployment,
		setErrorDetailDeployment,
		handleCancelDeployment,
		handleClearAllDeployments,
		filteredAndSorted,
		activeQueue,
		clearFilters,
		deployments,
	} = useDeployments();

	const [cancelingId, setCancelingId] = useState<number | null>(null);
	const navigate = useNavigate();
	const {id: deepLinkId, tab: tabParam} = useSearch({from: '/_app/Deployments'});
	const active: DeploymentsTabId = tabParam ?? 'history';

	const select = (id: DeploymentsTabId) =>
		void navigate({
			to: '/Deployments',
			search: {tab: id === 'history' ? undefined : id},
			replace: true,
		});

	// OpenShip-style fleet stats from the full deployment list.
	const stats = useMemo(() => {
		let success = 0;
		let failed = 0;
		let inProgress = 0;
		for (const d of deployments) {
			const s = (d.status || '').toUpperCase();
			if (s === 'DONE' || s === 'SUCCESS' || s === 'DEPLOYED' || s === 'HEALTHY')
				success++;
			else if (
				s === 'ERROR' ||
				s === 'FAILED' ||
				s === 'CRASHED' ||
				s === 'CANCELLED'
			)
				failed++;
			else if (d && !['IDLE', 'STOPPED', 'STOPPING'].includes(s))
				inProgress++;
		}
		return {total: deployments.length, success, failed, inProgress};
	}, [deployments]);

	// Deep link: /Deployments?id=<id> opens that deployment's log stream and
	// clears the param, so a refresh doesn't re-open the dialog.
	useEffect(() => {
		if (!deepLinkId) return;
		const match = deployments.find(d => String(d.id) === deepLinkId);
		if (match) setSelectedDeployment(match);
		void navigate({to: '/Deployments', search: {}, replace: true});
	}, [deepLinkId, deployments, setSelectedDeployment, navigate]);

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* Header — quiet title + live count line (OpenShip) */}
			<div className="mb-2 flex flex-wrap items-end justify-between gap-4">
				<div className="flex flex-col gap-1">
					<h1 className="text-2xl font-medium tracking-tight text-foreground/80">
						Deployments
					</h1>
					<p className="text-sm text-muted-foreground/70">
						{isLoading
							? 'Loading deployment history...'
							: `${stats.total} deployments across your infrastructure`}
					</p>
				</div>
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						onClick={handleRefresh}
						disabled={refreshing}
						className="flex h-9 items-center gap-2 rounded-xl text-xs font-semibold shadow-none">
						<RefreshCw
							className={cn('size-3.5', refreshing && 'animate-spin')}
						/>
						Refresh
					</Button>
					<Button
						variant="outline"
						onClick={handleClearAllDeployments}
						className="flex h-9 items-center gap-2 rounded-xl border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive shadow-none hover:bg-destructive/20">
						<Trash2 className="size-3.5" />
						Clear History
					</Button>
				</div>
			</div>

			{/* Mobile: pill strip */}
			<div className="overflow-x-auto lg:hidden">
				<div className="inline-flex items-center gap-1 rounded-xl bg-card p-1">
					{TABS.map(t => (
						<button
							key={t.id}
							type="button"
							onClick={() => select(t.id)}
							className={cn(
								'inline-flex items-center gap-2 whitespace-nowrap rounded-lg px-3 py-2 text-sm font-medium transition-colors',
								t.id === active
									? 'bg-foreground/[0.07] text-foreground'
									: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
							)}>
							<t.icon className="size-[15px]" strokeWidth={1.7} />
							{t.label}
							<span className="rounded-md bg-muted/60 px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
								{t.id === 'history'
									? filteredAndSorted.length
									: activeQueue.length}
							</span>
						</button>
					))}
				</div>
			</div>

			{/* Main Grid (OpenShip): content left, sticky overview rail right */}
			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
				{/* LEFT COLUMN */}
				<div className="min-w-0 space-y-4">
					{active === 'history' ? (
						<>
							<DeploymentsFilters
								searchQuery={searchQuery}
								setSearchQuery={setSearchQuery}
								statusFilter={statusFilter}
								setStatusFilter={setStatusFilter}
								typeFilter={typeFilter}
								setTypeFilter={setTypeFilter}
								sortBy={sortBy}
								sortDir={sortDir}
								setSortBy={setSortBy}
								setSortDir={setSortDir}
							/>

							{isLoading ? (
								<div className="flex flex-col items-center justify-center gap-3 py-20">
									<RefreshCw className="size-8 animate-spin text-primary" />
									<p className="animate-pulse text-sm font-medium text-muted-foreground">
										Loading deployments history...
									</p>
								</div>
							) : filteredAndSorted.length > 0 ? (
								<DeploymentActivityList
									deployments={filteredAndSorted}
									onViewLogs={setSelectedDeployment as any}
								/>
							) : (
								<EmptyState
									hasFilters={Boolean(
										searchQuery ||
											statusFilter !== 'all' ||
											typeFilter !== 'all',
									)}
									onClearFilters={clearFilters}
								/>
							)}
						</>
					) : (
						<>
							<div className="flex items-center justify-between">
								<h2 className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
									Live Build Queue Processing
								</h2>
								<span className="text-xs text-muted-foreground">
									{activeQueue.length > 0
										? 'Auto-refreshing every 1s'
										: 'Queue idle'}
								</span>
							</div>

							{activeQueue.length > 0 ? (
								<div className="animate-in overflow-hidden rounded-xl bg-muted/20 shadow-2xs duration-200 fade-in dark:bg-muted/15">
									<Table>
										<TableHeader className="bg-muted/40">
											<TableRow className="">
												<TableHead className="w-[80px] text-xs font-bold">
													ID
												</TableHead>
												<TableHead className="text-xs font-bold">
													Deployment Title
												</TableHead>
												<TableHead className="text-xs font-bold">
													Type
												</TableHead>
												<TableHead className="text-xs font-bold">
													Status
												</TableHead>
												<TableHead className="text-xs font-bold">
													Time
												</TableHead>
												<TableHead className="text-right text-xs font-bold">
													Actions
												</TableHead>
											</TableRow>
										</TableHeader>
										<TableBody>
											{activeQueue.map(d => {
												const hasApp =
													d.application_id !== null &&
													d.application_id !== undefined;
												const hasCompose =
													d.compose_id !== null &&
													d.compose_id !== undefined;
												const hasDatabase =
													d.database_id !== null &&
													d.database_id !== undefined;
												const type = hasApp
													? 'Application'
													: hasCompose
														? 'Compose'
														: hasDatabase
															? 'Database'
															: 'Generic';
												const status = (d.status || '').toUpperCase();

												return (
													<TableRow
														key={d.id}
														className=" hover:bg-muted/20">
														<TableCell className="font-mono text-xs font-semibold text-muted-foreground">
															#{d.id}
														</TableCell>
														<TableCell className="text-xs font-semibold text-foreground">
															<div className="flex flex-col">
																<span>
																	{d.title || `Deployment #${d.id}`}
																</span>
																{d.description && (
																	<span className="max-w-xs truncate text-[11px] font-normal text-muted-foreground">
																		{d.description}
																	</span>
																)}
															</div>
														</TableCell>
														<TableCell>
															<Badge
																variant="outline"
																className="font-mono text-[10px]">
																{type}
															</Badge>
														</TableCell>
														<TableCell>
															<Badge
																variant="outline"
																className="animate-pulse border-amber-500/30 bg-amber-500/10 text-[10px] font-bold tracking-wider text-amber-500 uppercase">
																{status}
															</Badge>
														</TableCell>
														<TableCell className="font-mono text-xs text-muted-foreground">
															<span className="flex items-center gap-1">
																<Clock className="h-3 w-3 text-muted-foreground" />
																{new Date(
																	Number(d.created_at || 0) * 1000,
																).toLocaleTimeString()}
															</span>
														</TableCell>
														<TableCell className="text-right">
															<div className="flex items-center justify-end gap-1.5">
																<Button
																	size="sm"
																	variant="outline"
																	onClick={() =>
																		setSelectedDeployment(d as any)
																	}
																	className="flex h-7 items-center gap-1 rounded-lg  px-2 text-xs font-semibold text-foreground hover:bg-muted">
																	<Terminal className="h-3 w-3" /> Stream Logs
																</Button>
																{d.id !== undefined && (
																	<Button
																		size="sm"
																		variant="outline"
																		onClick={() =>
																			setCancelingId(Number(d.id!))
																		}
																		className="flex h-7 items-center gap-1 rounded-lg border-destructive/20 px-2.5 text-xs font-semibold text-destructive hover:bg-destructive/10">
																		<XCircle className="h-3.5 w-3.5" /> Cancel
																		Build
																	</Button>
																)}
															</div>
														</TableCell>
													</TableRow>
												);
											})}
										</TableBody>
									</Table>
								</div>
							) : (
								<div className="flex flex-col items-center justify-center gap-2 rounded-xl bg-card p-6 text-center text-muted-foreground">
									<CheckCircle2 className="h-8 w-8 text-emerald-500 opacity-80" />
									<p className="text-sm font-semibold text-foreground">
										No Active Builds in Queue
									</p>
									<p className="text-xs">
										All application and compose deployments have
										completed successfully.
									</p>
								</div>
							)}
						</>
					)}
				</div>

				{/* RIGHT COLUMN (sticky): nav + Activity Overview + tip */}
				<div className="hidden space-y-4 lg:sticky lg:top-6 lg:block lg:self-start">
					{/* Section nav */}
					<div className="rounded-2xl bg-card p-3">
						<div className="space-y-1">
							{TABS.map(t => {
								const isActive = t.id === active;
								const count =
									t.id === 'history'
										? filteredAndSorted.length
										: activeQueue.length;
								return (
									<button
										key={t.id}
										type="button"
										onClick={() => select(t.id)}
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
													activeQueue.length > 0 &&
													'animate-spin text-amber-500',
											)}
											strokeWidth={1.7}
										/>
										<span className="flex-1 text-start">{t.label}</span>
										<span
											className={cn(
												'rounded-md px-1.5 py-0.5 font-mono text-[10px]',
												t.id === 'queue' && activeQueue.length > 0
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
					<div className="rounded-2xl border border-border/50 bg-card p-5">
						<div className="mb-4 flex items-center gap-2">
							<Activity className="size-4 text-muted-foreground" />
							<h3 className="text-sm font-semibold text-foreground">
								Activity Overview
							</h3>
						</div>

						<div className="space-y-3">
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div className="flex size-8 items-center justify-center rounded-lg bg-primary/10">
										<Layers className="size-4 text-primary" />
									</div>
									<span className="text-sm text-muted-foreground">
										Total
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading ? '–' : stats.total}
								</span>
							</div>

							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div className="flex size-8 items-center justify-center rounded-lg bg-emerald-500/10">
										<CheckCircle2 className="size-4 text-emerald-500" />
									</div>
									<span className="text-sm text-muted-foreground">
										Successful
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading ? '–' : stats.success}
								</span>
							</div>

							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div className="flex size-8 items-center justify-center rounded-lg bg-rose-500/10">
										<XCircle className="size-4 text-rose-500" />
									</div>
									<span className="text-sm text-muted-foreground">
										Failed
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading ? '–' : stats.failed}
								</span>
							</div>

							{stats.inProgress > 0 && (
								<div className="flex items-center justify-between">
									<div className="flex items-center gap-2">
										<div className="flex size-8 items-center justify-center rounded-lg bg-amber-500/10">
											<Loader2 className="size-4 animate-spin text-amber-500" />
										</div>
										<span className="text-sm text-muted-foreground">
											In progress
										</span>
									</div>
									<span className="text-lg font-semibold text-foreground">
										{stats.inProgress}
									</span>
								</div>
							)}
						</div>

						{/* Fleet-health proportion bar */}
						{!isLoading && stats.total > 0 && (
							<div className="mt-4 flex h-1.5 overflow-hidden rounded-full bg-muted/40">
								{stats.success > 0 && (
									<div
										className="bg-emerald-500"
										style={{
											width: `${(stats.success / stats.total) * 100}%`,
										}}
									/>
								)}
								{stats.failed > 0 && (
									<div
										className="bg-rose-500"
										style={{
											width: `${(stats.failed / stats.total) * 100}%`,
										}}
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

					{/* Quick tip (gradient card) */}
					<div className="rounded-2xl border border-primary/10 bg-gradient-to-br from-primary/5 via-primary/3 to-transparent p-5">
						<div className="mb-3 flex items-center gap-2">
							<Zap className="size-4 text-primary" />
							<h3 className="text-sm font-semibold text-foreground">
								{stats.total > 0 ? 'Auto-deploy' : 'Get started'}
							</h3>
						</div>
						<p className="text-sm leading-relaxed text-muted-foreground">
							{stats.total > 0
								? 'Enable Autodeploy on an application and every git push ships automatically — watch the builds land here in real time.'
								: 'Deploy your first application or database and its build stream will show up here, line by line.'}
						</p>
						<Link
							to="/projects"
							className="mt-3 inline-flex items-center gap-1 text-sm font-medium text-primary transition-colors hover:text-primary/80">
							{stats.total > 0 ? 'Go to projects' : 'Browse projects'}
							<ArrowRight className="size-3.5" />
						</Link>
					</div>
				</div>
			</div>

			{/* Logs Stream Dialog */}
			<DeploymentLogsDialog
				selectedDeployment={selectedDeployment}
				onClose={() => setSelectedDeployment(null)}
				logs={logs}
				isLogsLoading={isLogsLoading}
				copied={copied}
				onCopyLogs={handleCopyLogs}
			/>

			{/* Error Detail Dialog */}
			<DeploymentErrorDialog
				errorDetailDeployment={errorDetailDeployment}
				onClose={() => setErrorDetailDeployment(null)}
			/>

			{/* Cancel Confirmation Alert Dialog */}
			<AlertDialog
				open={cancelingId !== null}
				onOpenChange={open => !open && setCancelingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Cancel Deployment</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to cancel this deployment build? This
							action will terminate the build task.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setCancelingId(null)}>
							Keep Running
						</AlertDialogCancel>
						<AlertDialogAction
							className="text-destructive-foreground bg-destructive hover:bg-destructive/90"
							onClick={async () => {
								if (cancelingId) {
									await handleCancelDeployment(cancelingId);
									setCancelingId(null);
								}
							}}>
							Yes, Cancel Build
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
