import {createFileRoute} from '@tanstack/react-router';
import {RefreshCw, FileText, Activity, CheckCircle2, Layers, Terminal, XCircle, Clock} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Tabs, TabsList, TabsTrigger, TabsContent} from '#/components/ui/tabs';
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
		<div className="flex flex-col gap-6 w-full pb-10">
			{/* Main Page Header */}
			<DeploymentsHeader refreshing={refreshing} onRefresh={handleRefresh} onClearAll={handleClearAllDeployments} />

			{/* Interactive Tabs for History & Queue */}
			<Tabs defaultValue="history" className="w-full space-y-6">
				<TabsList variant="line" className="border-b border-border w-full justify-start gap-6 rounded-none pb-0">
					<TabsTrigger value="history" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Layers className="w-3.5 h-3.5" />
						Logs & History
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{filteredAndSorted.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="queue" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Activity className={`w-3.5 h-3.5 ${activeQueue.length > 0 ? 'text-amber-500 animate-spin' : ''}`} />
						Active Queue
						<Badge
							variant={activeQueue.length > 0 ? 'default' : 'secondary'}
							className={`text-[10px] font-mono px-1.5 py-0 ${activeQueue.length > 0 ? 'bg-amber-500 text-black font-bold animate-pulse' : ''}`}>
							{activeQueue.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				{/* TAB 1: Deployment Logs & History Content */}
				<TabsContent value="history" className="space-y-4 focus-visible:outline-none">
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
						<div className="flex flex-col gap-3 py-20 items-center justify-center">
							<RefreshCw className="size-8 animate-spin text-primary" />
							<p className="text-sm text-muted-foreground font-medium animate-pulse">
								Loading deployments history...
							</p>
						</div>
					) : filteredAndSorted.length > 0 ? (
						<section className="bg-card border border-border rounded-xl overflow-hidden shadow-2xs animate-in fade-in duration-200 flex flex-col">
							<div className="max-h-[calc(100vh-320px)] min-h-[320px] overflow-y-auto">
								<Table>
									<TableHeader className="sticky top-0 z-20">
										<TableRow className="border-border hover:bg-transparent">
											<TableHead className="w-[80px] h-10 text-xs font-medium text-muted-foreground bg-muted/50 sticky top-0 z-20 border-b border-border">ID</TableHead>
											<TableHead className="h-10 text-xs font-medium text-muted-foreground bg-muted/50 sticky top-0 z-20 border-b border-border">Deployment Title</TableHead>
											<TableHead className="h-10 text-xs font-medium text-muted-foreground bg-muted/50 sticky top-0 z-20 border-b border-border">Type</TableHead>
											<TableHead className="h-10 text-xs font-medium text-muted-foreground bg-muted/50 sticky top-0 z-20 border-b border-border">Status</TableHead>
											<TableHead className="h-10 text-xs font-medium text-muted-foreground bg-muted/50 sticky top-0 z-20 border-b border-border">Date</TableHead>
											<TableHead className="text-right h-10 text-xs font-medium text-muted-foreground bg-muted/50 sticky top-0 z-20 border-b border-border">Actions</TableHead>
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
						<div className="flex flex-col items-center justify-center border border-dashed border-border/40 rounded-2xl py-20 text-center bg-muted/10">
							<FileText className="size-12 opacity-20 text-muted-foreground" />
							<h3 className="text-md font-bold text-foreground mt-3">No deployment logs found</h3>
							<p className="text-muted-foreground mt-1 text-xs max-w-sm">
								{searchQuery || statusFilter !== 'all' || typeFilter !== 'all'
									? 'No records match your filters. Try clearing search options.'
									: 'No deployment events registered in this system yet.'}
							</p>
							{(searchQuery || statusFilter !== 'all' || typeFilter !== 'all') && (
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
				<TabsContent value="queue" className="space-y-4 focus-visible:outline-none">
					<div className="flex items-center justify-between">
						<h2 className="text-xs font-bold text-muted-foreground uppercase tracking-wider">
							Live Build Queue Processing
						</h2>
						<span className="text-xs text-muted-foreground">
							{activeQueue.length > 0 ? 'Auto-refreshing every 1s' : 'Queue idle'}
						</span>
					</div>

					{activeQueue.length > 0 ? (
						<div className="bg-muted/20 dark:bg-muted/15 border border-border/40 rounded-xl overflow-hidden shadow-2xs animate-in fade-in duration-200">
							<Table>
								<TableHeader className="bg-muted/40">
									<TableRow className="border-border/40">
										<TableHead className="w-[80px] text-xs font-bold">ID</TableHead>
										<TableHead className="text-xs font-bold">Deployment Title</TableHead>
										<TableHead className="text-xs font-bold">Type</TableHead>
										<TableHead className="text-xs font-bold">Status</TableHead>
										<TableHead className="text-xs font-bold">Time</TableHead>
										<TableHead className="text-right text-xs font-bold">Actions</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{activeQueue.map(d => {
										const hasApp = d.application_id !== null && d.application_id !== undefined;
										const hasCompose = d.compose_id !== null && d.compose_id !== undefined;
										const hasDatabase = d.database_id !== null && d.database_id !== undefined;
										const type = hasApp ? 'Application' : hasCompose ? 'Compose' : hasDatabase ? 'Database' : 'Generic';
										const status = (d.status || '').toUpperCase();

										return (
											<TableRow key={d.id} className="border-border hover:bg-muted/20">
												<TableCell className="font-mono text-xs font-semibold text-muted-foreground">
													#{d.id}
												</TableCell>
												<TableCell className="font-semibold text-xs text-foreground">
													<div className="flex flex-col">
														<span>{d.title || `Deployment #${d.id}`}</span>
														{d.description && (
															<span className="text-[11px] font-normal text-muted-foreground truncate max-w-xs">
																{d.description}
															</span>
														)}
													</div>
												</TableCell>
												<TableCell>
													<Badge variant="outline" className="text-[10px] font-mono">
														{type}
													</Badge>
												</TableCell>
												<TableCell>
													<Badge variant="outline" className="text-[10px] font-bold uppercase tracking-wider text-amber-500 border-amber-500/30 bg-amber-500/10 animate-pulse">
														{status}
													</Badge>
												</TableCell>
												<TableCell className="text-xs text-muted-foreground font-mono">
													<span className="flex items-center gap-1">
														<Clock className="w-3 h-3 text-muted-foreground" />
														{new Date(Number(d.created_at || 0) * 1000).toLocaleTimeString()}
													</span>
												</TableCell>
												<TableCell className="text-right">
													<div className="flex items-center justify-end gap-1.5">
														<Button
															size="sm"
															variant="outline"
															onClick={() => setSelectedDeployment(d as any)}
															className="h-7 text-xs border-border text-foreground hover:bg-muted px-2 rounded-lg font-semibold flex items-center gap-1">
															<Terminal className="w-3 h-3" /> Stream Logs
														</Button>
														{d.id !== undefined && (
															<Button
																size="sm"
																variant="outline"
																onClick={() => setCancelingId(Number(d.id!))}
																className="h-7 text-xs text-destructive border-destructive/20 hover:bg-destructive/10 px-2.5 rounded-lg font-semibold flex items-center gap-1">
																<XCircle className="w-3.5 h-3.5" /> Cancel Build
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
						<div className="bg-card border border-border rounded-xl p-6 flex flex-col items-center justify-center text-center gap-2 text-muted-foreground">
							<CheckCircle2 className="w-8 h-8 text-emerald-500 opacity-80" />
							<p className="text-sm font-semibold text-foreground">No Active Builds in Queue</p>
							<p className="text-xs">All application and compose deployments have completed successfully.</p>
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
			<AlertDialog open={cancelingId !== null} onOpenChange={open => !open && setCancelingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Cancel Deployment</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to cancel this deployment build? This action will terminate the build task.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setCancelingId(null)}>Keep Running</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={async () => {
								if (cancelingId) {
									await handleCancelDeployment(cancelingId);
									setCancelingId(null);
								}
							}}
						>
							Yes, Cancel Build
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
