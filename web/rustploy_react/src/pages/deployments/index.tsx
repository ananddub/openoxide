import {createFileRoute} from '@tanstack/react-router';
import {RefreshCw, FileText} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useDeployments} from '#/hooks/deployments/use-deployments';
import {DeploymentsHeader} from '#/components/deployments/deployments-header';
import {DeploymentsFilters} from '#/components/deployments/deployments-filters';
import {DeploymentItem} from '#/components/deployments/deployment-item';
import {DeploymentLogsDialog} from '#/components/deployments/deployment-logs-dialog';
import {DeploymentErrorDialog} from '#/components/deployments/deployment-error-dialog';

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
		filteredAndSorted,
		clearFilters,
	} = useDeployments();

	return (
		<div className="flex flex-col gap-6 w-full pb-10">
			{/* Page Header */}
			<DeploymentsHeader refreshing={refreshing} onRefresh={handleRefresh} />

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

			{/* Deployments List */}
			{isLoading ? (
				<div className="flex flex-col gap-3 py-20 items-center justify-center">
					<RefreshCw className="size-8 animate-spin text-primary" />
					<p className="text-sm text-muted-foreground font-medium animate-pulse">
						Loading deployments...
					</p>
				</div>
			) : filteredAndSorted.length > 0 ? (
				<div className="flex flex-col gap-3 animate-in fade-in duration-200">
					{filteredAndSorted.map(d => (
						<DeploymentItem
							key={d.id}
							deployment={d}
							onViewLogs={setSelectedDeployment}
							onViewError={setErrorDetailDeployment}
							onCancel={handleCancelDeployment}
						/>
					))}
				</div>
			) : (
				<div className="flex flex-col items-center justify-center border border-dashed border-border/80 rounded-2xl py-20 text-center bg-card/10 backdrop-blur-[2px]">
					<FileText className="size-12 opacity-20 text-muted-foreground" />
					<h3 className="text-md font-bold text-foreground mt-3">No deployments found</h3>
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
		</div>
	);
}
