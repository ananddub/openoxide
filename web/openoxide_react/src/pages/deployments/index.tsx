import {createFileRoute} from '@tanstack/react-router';
import {
	RefreshCw,
	FileText,
	Activity,
	CheckCircle2,
	Layers,
	Terminal,
	XCircle,
	Clock,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	Tabs,
	TabsList,
	TabsTrigger,
	TabsContent,
} from '#/components/ui/tabs';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import {useDeployments} from '#/hooks/deployments/use-deployments';
import {DeploymentsHeader} from '#/components/deployments/deployments-header';
import {DeploymentsFilters} from '#/components/deployments/deployments-filters';
import {DeploymentItem} from '#/components/deployments/deployment-item';
import {DeploymentLogsDialog} from '#/components/deployments/deployment-logs-dialog';
import {DeploymentErrorDialog} from '#/components/deployments/deployment-error-dialog';
import {useState} from 'react';
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

export const Route = createFileRoute('/_app/Deployments')({
	component: DeploymentsPage,
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
		handleDeleteDeployment,
		handleClearAllDeployments,
		filteredAndSorted,
		activeQueue,
		clearFilters,
	} = useDeployments();

	const [cancelingId, setCancelingId] = useState<number | null>(null);

	return (
		<div className="flex w-full flex-col gap-6 pb-10">
			{/* Main Page Header */}
			<DeploymentsHeader
				refreshing={refreshing}
				onRefresh={handleRefresh}
				onClearAll={handleClearAllDeployments}
			/>

			{/* Interactive Tabs for History & Queue */}
			<Tabs defaultValue="history" className="w-full space-y-6">
				<TabsList
					variant="line"
					className="w-full justify-start gap-6 rounded-none border-b border-border pb-0">
					<TabsTrigger
						value="history"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Layers className="h-3.5 w-3.5" />
						Logs & History
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
							{filteredAndSorted.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger
						value="queue"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Activity
							className={`h-3.5 w-3.5 ${activeQueue.length > 0 ? 'animate-spin text-amber-500' : ''}`}
						/>
						Active Queue
						<Badge
							variant={activeQueue.length > 0 ? 'default' : 'secondary'}
							className={`px-1.5 py-0 font-mono text-[10px] ${activeQueue.length > 0 ? 'animate-pulse bg-amber-500 font-bold text-black' : ''}`}>
							{activeQueue.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				{/* TAB 1: Deployment Logs & History Content */}
				<TabsContent
					value="history"
					className="space-y-4 focus-visible:outline-none">
					{/* Filters Control Bar */}
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

					{/* Deployments History List */}
					{isLoading ? (
						<div className="flex flex-col items-center justify-center gap-3 py-20">
							<RefreshCw className="size-8 animate-spin text-primary" />
							<p className="animate-pulse text-sm font-medium text-muted-foreground">
								Loading deployments history...
							</p>
						</div>
					) : filteredAndSorted.length > 0 ? (
						<section className="flex animate-in flex-col overflow-hidden rounded-xl border border-border bg-card shadow-2xs duration-200 fade-in">
							<div className="max-h-[calc(100vh-320px)] min-h-[320px] overflow-y-auto">
								<Table>
									<TableHeader className="sticky top-0 z-20">
										<TableRow className="border-border hover:bg-transparent">
											<TableHead className="sticky top-0 z-20 h-10 w-[80px] border-b border-border bg-muted/50 text-xs font-medium text-muted-foreground">
												ID
											</TableHead>
											<TableHead className="sticky top-0 z-20 h-10 border-b border-border bg-muted/50 text-xs font-medium text-muted-foreground">
												Deployment Title
											</TableHead>
											<TableHead className="sticky top-0 z-20 h-10 border-b border-border bg-muted/50 text-xs font-medium text-muted-foreground">
												Type
											</TableHead>
											<TableHead className="sticky top-0 z-20 h-10 border-b border-border bg-muted/50 text-xs font-medium text-muted-foreground">
												Status
											</TableHead>
											<TableHead className="sticky top-0 z-20 h-10 border-b border-border bg-muted/50 text-xs font-medium text-muted-foreground">
												Date
											</TableHead>
											<TableHead className="sticky top-0 z-20 h-10 border-b border-border bg-muted/50 text-right text-xs font-medium text-muted-foreground">
												Actions
											</TableHead>
										</TableRow>
									</TableHeader>
									<TableBody>
										{filteredAndSorted.map(d => (
											<DeploymentItem
												key={String(d.id)}
												deployment={d as any}
												onViewLogs={setSelectedDeployment as any}
												onViewError={setErrorDetailDeployment as any}
												onCancel={handleCancelDeployment}
												onDelete={handleDeleteDeployment}
											/>
										))}
									</TableBody>
								</Table>
							</div>
						</section>
					) : (
						<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/40 bg-muted/10 py-20 text-center">
							<FileText className="size-12 text-muted-foreground opacity-20" />
							<h3 className="text-md mt-3 font-bold text-foreground">
								No deployment logs found
							</h3>
							<p className="mt-1 max-w-sm text-xs text-muted-foreground">
								{searchQuery ||
								statusFilter !== 'all' ||
								typeFilter !== 'all'
									? 'No records match your filters. Try clearing search options.'
									: 'No deployment events registered in this system yet.'}
							</p>
							{(searchQuery ||
								statusFilter !== 'all' ||
								typeFilter !== 'all') && (
								<Button
									variant="ghost"
									onClick={clearFilters}
									className="mt-4 text-xs font-semibold text-primary">
									Clear All Filters
								</Button>
							)}
						</div>
					)}
				</TabsContent>

				{/* TAB 2: Active Queue Content */}
				<TabsContent
					value="queue"
					className="space-y-4 focus-visible:outline-none">
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
						<div className="animate-in overflow-hidden rounded-xl border border-border/40 bg-muted/20 shadow-2xs duration-200 fade-in dark:bg-muted/15">
							<Table>
								<TableHeader className="bg-muted/40">
									<TableRow className="border-border/40">
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
											d.compose_id !== null && d.compose_id !== undefined;
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
												className="border-border hover:bg-muted/20">
												<TableCell className="font-mono text-xs font-semibold text-muted-foreground">
													#{d.id}
												</TableCell>
												<TableCell className="text-xs font-semibold text-foreground">
													<div className="flex flex-col">
														<span>{d.title || `Deployment #${d.id}`}</span>
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
															className="flex h-7 items-center gap-1 rounded-lg border-border px-2 text-xs font-semibold text-foreground hover:bg-muted">
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
						<div className="flex flex-col items-center justify-center gap-2 rounded-xl border border-border bg-card p-6 text-center text-muted-foreground">
							<CheckCircle2 className="h-8 w-8 text-emerald-500 opacity-80" />
							<p className="text-sm font-semibold text-foreground">
								No Active Builds in Queue
							</p>
							<p className="text-xs">
								All application and compose deployments have completed
								successfully.
							</p>
						</div>
					)}
				</TabsContent>
			</Tabs>

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
