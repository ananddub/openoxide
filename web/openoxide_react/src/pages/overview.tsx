import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { Box, Globe, Clock, Rocket } from 'lucide-react';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '#/components/ui/tabs';
import { Badge } from '#/components/ui/badge';
import { useAppStore } from '#/stores/app-store';
import { useDeployments } from '#/hooks/deployments/use-deployments';
import { DeploymentItem } from '#/components/deployments/deployment-item';
import { DeploymentLogsDialog } from '#/components/deployments/deployment-logs-dialog';
import { OverviewServicesTab } from '#/components/overview/overview-services-tab';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState('services');

	const domains = useAppStore((state) => state.domains);
	const backups = useAppStore((state) => state.backups);

	const {
		filteredAndSorted: deploymentsList,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		copied,
		handleCopyLogs,
	} = useDeployments();

	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			{/* Page Header */}
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Overview</h1>
				<p className="text-xs text-muted-foreground">
					Centralized platform overview of all applications, compose stacks, databases, deployments, domains, and backups
				</p>
			</div>

			{/* Main Overview Tabs */}
			<Tabs value={activeTab} onValueChange={setActiveTab} className="w-full space-y-6">
				<TabsList variant="line" className="border-b border-border w-full justify-start gap-6 rounded-none pb-0">
					<TabsTrigger value="services" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Box className="size-4" />
						Services
					</TabsTrigger>
					<TabsTrigger value="deployments" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Rocket className="size-4" />
						Deployments
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{deploymentsList.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="domains" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Globe className="size-4" />
						Domains
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{domains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="backups" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Clock className="size-4" />
						Volume Backups
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{backups.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				{/* Services Tab */}
				<TabsContent value="services">
					<OverviewServicesTab />
				</TabsContent>

				{/* Deployments Tab */}
				<TabsContent value="deployments" className="space-y-4">
					{deploymentsList.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Rocket className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No deployment history</p>
							<p className="text-xs text-muted-foreground">Deployments will appear here once triggered</p>
						</div>
					) : (
						<div className="flex flex-col gap-2">
							{deploymentsList.map((dep) => (
								<DeploymentItem
									key={dep.id}
									deployment={dep}
									onViewLogs={() => setSelectedDeployment(dep)}
									onViewError={() => setSelectedDeployment(dep)}
								/>
							))}
						</div>
					)}
				</TabsContent>

				{/* Domains Tab */}
				<TabsContent value="domains" className="space-y-4">
					{domains.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Globe className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No active domains configured</p>
							<p className="text-xs text-muted-foreground">Configure custom domain routes in application settings</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{domains.map((dom) => (
								<div key={dom.id} className="p-4 border rounded-xl bg-card flex flex-col gap-2">
									<div className="flex items-center justify-between">
										<p className="font-bold text-xs text-foreground font-mono">{dom.domain || dom.host}</p>
										<Badge variant="outline" className="text-[10px]">
											{dom.https ? 'HTTPS' : 'HTTP'}
										</Badge>
									</div>
									<p className="text-[10px] text-muted-foreground font-mono">
										Port: {dom.port || 80} · Path: {dom.path || '/'}
									</p>
								</div>
							))}
						</div>
					)}
				</TabsContent>

				{/* Backups Tab */}
				<TabsContent value="backups" className="space-y-4">
					{backups.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Clock className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No volume backups found</p>
							<p className="text-xs text-muted-foreground">Scheduled volume backups will appear here</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{backups.map((b) => (
								<div key={b.id} className="p-4 border rounded-xl bg-card flex flex-col gap-2">
									<div className="flex items-center justify-between">
										<p className="font-bold text-xs text-foreground font-mono">{b.name || `Backup #${b.id}`}</p>
										<Badge variant="secondary" className="text-[10px]">
											{b.status || 'DONE'}
										</Badge>
									</div>
									<p className="text-[10px] text-muted-foreground font-mono">
										Size: {b.size || 'N/A'} · Dest: {b.destination || 'Local'}
									</p>
								</div>
							))}
						</div>
					)}
				</TabsContent>
			</Tabs>

			{/* Logs Dialog */}
			<DeploymentLogsDialog
				selectedDeployment={selectedDeployment}
				onClose={() => setSelectedDeployment(null)}
				logs={logs}
				isLogsLoading={false}
				copied={copied}
				onCopyLogs={handleCopyLogs}
			/>
		</div>
	);
}
