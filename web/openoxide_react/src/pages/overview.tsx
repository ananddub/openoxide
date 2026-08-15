import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { Box, Globe, Clock, Rocket, Loader2, ExternalLink } from 'lucide-react';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '#/components/ui/tabs';
import { Badge } from '#/components/ui/badge';
import { Button } from '#/components/ui/button';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import { useAppStore } from '#/stores/app-store';
import { useDeployments } from '#/hooks/deployments/use-deployments';
import { DeploymentItem } from '#/components/deployments/deployment-item';
import { DeploymentLogsDialog } from '#/components/deployments/deployment-logs-dialog';
import { OverviewServicesTab } from '#/components/overview/overview-services-tab';
import { useDeploymentRunning } from 'virtual:openoxide-live';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState('services');
	const [deploymentSubTab, setDeploymentSubTab] = useState<'history' | 'queue'>('history');

	const domains = useAppStore((state) => state.domains);
	const backups = useAppStore((state) => state.backups);

	// Live running deployments queue
	const { data: runningDeployments = [] } = useDeploymentRunning();

	const {
		filteredAndSorted: deploymentsList,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		copied,
		handleCopyLogs,
		handleCancelDeployment,
	} = useDeployments();

	return (
		<div className="p-6 md:px-8 flex flex-col gap-6 w-full max-w-full animate-in fade-in duration-200">
			{/* Page Header */}
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Overview</h1>
				<p className="text-xs text-muted-foreground">
					Centralized platform overview of all applications, compose stacks, databases, volume backups, domains, and deployments
				</p>
			</div>

			{/* Main Overview Tabs (Services -> Backups -> Domains -> Deployments) */}
			<Tabs value={activeTab} onValueChange={setActiveTab} className="w-full space-y-6">
				<TabsList variant="line" className="border-b border-border w-full justify-start gap-6 rounded-none pb-0">
					<TabsTrigger value="services" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Box className="size-4" />
						Services
					</TabsTrigger>
					<TabsTrigger value="backups" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Clock className="size-4" />
						Backups
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{backups.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="domains" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Globe className="size-4" />
						Domains
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{domains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="deployments" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Rocket className="size-4" />
						Deployments
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{deploymentsList.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				{/* 1. Services Tab */}
				<TabsContent value="services">
					<OverviewServicesTab />
				</TabsContent>

				{/* 2. Backups Tab Table */}
				<TabsContent value="backups" className="space-y-4">
					{backups.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Clock className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No volume backups found</p>
							<p className="text-xs text-muted-foreground">Scheduled volume and database backups will appear here</p>
						</div>
					) : (
						<div className="rounded-xl border border-border/60 bg-card overflow-y-auto max-h-[calc(100vh-340px)] min-h-[280px] shadow-xs">
							<Table>
								<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
									<TableRow className="border-b border-border/60 hover:bg-transparent">
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Backup Name</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Type</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Status</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Destination</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Created</TableHead>
										<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Actions</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{backups.map((b) => (
										<TableRow key={b.id} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
											<TableCell className="py-3.5 px-4 font-bold text-xs text-foreground font-mono">
												<div className="flex items-center gap-2">
													<Clock className="size-4 text-primary shrink-0" />
													<span>{b.name || `Backup #${b.id}`}</span>
												</div>
											</TableCell>
											<TableCell className="py-3.5 px-4 text-xs font-semibold text-foreground">
												<Badge variant="outline" className="text-[10px] font-mono">
													{b.backup_type || 'Volume'}
												</Badge>
											</TableCell>
											<TableCell className="py-3.5 px-4">
												<div className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-500/10 text-xs font-semibold text-emerald-500">
													<span className="size-2 rounded-full bg-emerald-500 shrink-0" />
													<span>{b.status || 'DONE'}</span>
												</div>
											</TableCell>
											<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
												{b.destination || 'Local Storage'}
											</TableCell>
											<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
												{b.created_at ? new Date(Number(b.created_at) * 1000).toLocaleDateString() : 'N/A'}
											</TableCell>
											<TableCell className="py-3.5 px-4 text-right">
												<Button size="sm" variant="outline" className="h-7 text-xs font-semibold px-2.5">
													Download
												</Button>
											</TableCell>
										</TableRow>
									))}
								</TableBody>
							</Table>
						</div>
					)}
				</TabsContent>

				{/* 3. Domains Tab Table */}
				<TabsContent value="domains" className="space-y-4">
					{domains.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Globe className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No active domains configured</p>
							<p className="text-xs text-muted-foreground">Configure custom domain routes in application settings</p>
						</div>
					) : (
						<div className="rounded-xl border border-border/60 bg-card overflow-y-auto max-h-[calc(100vh-340px)] min-h-[280px] shadow-xs">
							<Table>
								<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
									<TableRow className="border-b border-border/60 hover:bg-transparent">
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Domain / Host</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Target Service</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Path & Port</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">SSL Status</TableHead>
										<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Project</TableHead>
										<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Actions</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{domains.map((dom) => {
										const fullUrl = `${dom.https ? 'https' : 'http'}://${dom.domain || dom.host}${dom.path || ''}`;
										return (
											<TableRow key={dom.id} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
												<TableCell className="py-3.5 px-4 font-bold text-xs text-foreground font-mono">
													<div className="flex items-center gap-2">
														<Globe className="size-4 text-primary shrink-0" />
														<a
															href={fullUrl}
															target="_blank"
															rel="noreferrer"
															className="hover:underline hover:text-primary transition-colors"
														>
															{dom.domain || dom.host}
														</a>
													</div>
												</TableCell>
												<TableCell className="py-3.5 px-4 text-xs font-semibold text-foreground">
													{dom.service_name || 'Application'}
												</TableCell>
												<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
													{dom.path || '/'} · Port: {dom.port || 80}
												</TableCell>
												<TableCell className="py-3.5 px-4">
													{dom.https ? (
														<Badge variant="outline" className="text-[10px] font-semibold text-emerald-500 border-emerald-500/30 bg-emerald-500/10">
															HTTPS (SSL)
														</Badge>
													) : (
														<Badge variant="secondary" className="text-[10px] font-semibold">
															HTTP
														</Badge>
													)}
												</TableCell>
												<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-medium">
													{dom.project_name || 'Rustploy Project'}
												</TableCell>
												<TableCell className="py-3.5 px-4 text-right">
													<a
														href={fullUrl}
														target="_blank"
														rel="noreferrer"
														className="inline-flex items-center gap-1 text-xs font-semibold text-primary hover:underline"
													>
														<ExternalLink className="size-3.5" />
														<span>Visit</span>
													</a>
												</TableCell>
											</TableRow>
										);
									})}
								</TableBody>
							</Table>
						</div>
					)}
				</TabsContent>

				{/* 4. Deployments Tab (Deployments History vs Queue Sub-tabs) */}
				<TabsContent value="deployments" className="space-y-4">
					<div className="flex items-center gap-2 border-b border-border/60 pb-3">
						<Button
							variant={deploymentSubTab === 'history' ? 'secondary' : 'ghost'}
							size="sm"
							onClick={() => setDeploymentSubTab('history')}
							className="h-8 text-xs font-bold"
						>
							Deployments ({deploymentsList.length})
						</Button>
						<Button
							variant={deploymentSubTab === 'queue' ? 'secondary' : 'ghost'}
							size="sm"
							onClick={() => setDeploymentSubTab('queue')}
							className="h-8 text-xs font-bold flex items-center gap-1.5"
						>
							{runningDeployments.length > 0 && <Loader2 className="size-3 animate-spin text-amber-500" />}
							Queue ({runningDeployments.length})
						</Button>
					</div>

					{deploymentSubTab === 'history' ? (
						deploymentsList.length === 0 ? (
							<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
								<Rocket className="size-8 text-muted-foreground/40 mb-2" />
								<p className="text-sm font-semibold text-foreground">No deployment history</p>
								<p className="text-xs text-muted-foreground">Deployments will appear here once triggered</p>
							</div>
						) : (
							<div className="rounded-xl border border-border/60 bg-card overflow-y-auto max-h-[calc(100vh-340px)] min-h-[280px] shadow-xs">
								<Table>
									<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
										<TableRow className="border-b border-border/60 hover:bg-transparent">
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider w-[80px] bg-card/95 backdrop-blur-md sticky top-0 z-20">ID</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Deployment</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Type</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Status</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Created</TableHead>
											<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Actions</TableHead>
										</TableRow>
									</TableHeader>
									<TableBody>
										{deploymentsList.map((dep) => (
											<DeploymentItem
												key={dep.id}
												deployment={dep}
												onViewLogs={() => setSelectedDeployment(dep)}
												onViewError={() => setSelectedDeployment(dep)}
												onCancel={handleCancelDeployment}
											/>
										))}
									</TableBody>
								</Table>
							</div>
						)
					) : (
						runningDeployments.length === 0 ? (
							<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
								<Rocket className="size-8 text-muted-foreground/40 mb-2" />
								<p className="text-sm font-semibold text-foreground">Queue is empty</p>
								<p className="text-xs text-muted-foreground">Active or building deployments will appear here in real time</p>
							</div>
						) : (
							<div className="rounded-xl border border-border/60 bg-card overflow-y-auto max-h-[calc(100vh-340px)] min-h-[280px] shadow-xs">
								<Table>
									<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
										<TableRow className="border-b border-border/60 hover:bg-transparent">
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider w-[80px] bg-card/95 backdrop-blur-md sticky top-0 z-20">ID</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Deployment</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Type</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Status</TableHead>
											<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Created</TableHead>
											<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider bg-card/95 backdrop-blur-md sticky top-0 z-20">Actions</TableHead>
										</TableRow>
									</TableHeader>
									<TableBody>
										{runningDeployments.map((dep: any) => (
											<DeploymentItem
												key={dep.id}
												deployment={dep}
												onViewLogs={() => setSelectedDeployment(dep)}
												onViewError={() => setSelectedDeployment(dep)}
												onCancel={handleCancelDeployment}
											/>
										))}
									</TableBody>
								</Table>
							</div>
						)
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
