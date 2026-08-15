import { createFileRoute } from '@tanstack/react-router';
import { useState, useMemo } from 'react';
import { Box, Globe, Clock, Rocket, Loader2, ExternalLink, Search } from 'lucide-react';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '#/components/ui/tabs';
import { Badge } from '#/components/ui/badge';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import {
	Select,
	SelectTrigger,
	SelectValue,
	SelectContent,
	SelectItem,
} from '#/components/ui/select';
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

	// Backups Toolbar State
	const [backupSearch, setBackupSearch] = useState('');
	const [backupTypeFilter, setBackupTypeFilter] = useState('all');
	const [backupSort, setBackupSort] = useState('newest');

	// Domains Toolbar State
	const [domainSearch, setDomainSearch] = useState('');
	const [domainSslFilter, setDomainSslFilter] = useState('all');
	const [domainSort, setDomainSort] = useState('newest');

	// Deployments Toolbar State
	const [deploySearch, setDeploySearch] = useState('');
	const [deployStatusFilter, setDeployStatusFilter] = useState('all');

	const rawDomains = useAppStore((state) => state.domains);
	const rawBackups = useAppStore((state) => state.backups);

	// Live running deployments queue
	const { data: runningDeployments = [] } = useDeploymentRunning();

	const {
		filteredAndSorted: rawDeploymentsList,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		copied,
		handleCopyLogs,
		handleCancelDeployment,
	} = useDeployments();

	// Filter & Sort Backups
	const filteredBackups = useMemo(() => {
		let list = (rawBackups || []).filter((b: any) => {
			const matchesSearch =
				!backupSearch ||
				(b.name || '').toLowerCase().includes(backupSearch.toLowerCase()) ||
				(b.destination || '').toLowerCase().includes(backupSearch.toLowerCase());

			const matchesType =
				backupTypeFilter === 'all' ||
				(backupTypeFilter === 'volume' && (b.backup_type || '').toLowerCase().includes('volume')) ||
				(backupTypeFilter === 'database' && (b.backup_type || '').toLowerCase().includes('database'));

			return matchesSearch && matchesType;
		});

		if (backupSort === 'newest') list.sort((a, b) => Number(b.created_at || 0) - Number(a.created_at || 0));
		else if (backupSort === 'oldest') list.sort((a, b) => Number(a.created_at || 0) - Number(b.created_at || 0));

		return list;
	}, [rawBackups, backupSearch, backupTypeFilter, backupSort]);

	// Filter & Sort Domains
	const filteredDomains = useMemo(() => {
		let list = (rawDomains || []).filter((d: any) => {
			const hostStr = (d.domain || d.host || '').toLowerCase();
			const serviceStr = (d.service_name || '').toLowerCase();
			const projectStr = (d.project_name || '').toLowerCase();

			const matchesSearch =
				!domainSearch ||
				hostStr.includes(domainSearch.toLowerCase()) ||
				serviceStr.includes(domainSearch.toLowerCase()) ||
				projectStr.includes(domainSearch.toLowerCase());

			const matchesSsl =
				domainSslFilter === 'all' ||
				(domainSslFilter === 'https' && d.https) ||
				(domainSslFilter === 'http' && !d.https);

			return matchesSearch && matchesSsl;
		});

		if (domainSort === 'newest') list.sort((a, b) => Number(b.id || 0) - Number(a.id || 0));
		else if (domainSort === 'oldest') list.sort((a, b) => Number(a.id || 0) - Number(b.id || 0));

		return list;
	}, [rawDomains, domainSearch, domainSslFilter, domainSort]);

	// Filter Deployments History
	const filteredDeploymentsHistory = useMemo(() => {
		return (rawDeploymentsList || []).filter((d: any) => {
			const titleStr = (d.title || d.description || '').toLowerCase();
			const matchesSearch = !deploySearch || titleStr.includes(deploySearch.toLowerCase());

			const statusStr = (d.status || '').toLowerCase();
			const matchesStatus =
				deployStatusFilter === 'all' ||
				(deployStatusFilter === 'success' && (statusStr === 'done' || statusStr === 'healthy' || statusStr === 'success')) ||
				(deployStatusFilter === 'failed' && (statusStr === 'error' || statusStr === 'failed')) ||
				(deployStatusFilter === 'running' && (statusStr === 'running' || statusStr === 'queued'));

			return matchesSearch && matchesStatus;
		});
	}, [rawDeploymentsList, deploySearch, deployStatusFilter]);

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
							{filteredBackups.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="domains" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Globe className="size-4" />
						Domains
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{filteredDomains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="deployments" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Rocket className="size-4" />
						Deployments
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{filteredDeploymentsHistory.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				{/* 1. Services Tab */}
				<TabsContent value="services">
					<OverviewServicesTab />
				</TabsContent>

				{/* 2. Backups Tab Table with Dokploy Toolbar */}
				<TabsContent value="backups" className="space-y-4">
					{/* Toolbar Filters */}
					<div className="flex flex-wrap items-center justify-between gap-4">
						<h3 className="text-lg font-bold tracking-tight text-foreground">
							Backups{' '}
							<span className="text-sm font-normal text-muted-foreground">
								({filteredBackups.length})
							</span>
						</h3>

						<div className="flex flex-wrap items-center gap-2.5">
							{/* Search */}
							<div className="relative">
								<Input
									placeholder="Filter backups..."
									value={backupSearch}
									onChange={(e) => setBackupSearch(e.target.value)}
									className="pr-9 w-[190px] h-9 text-xs"
								/>
								<Search className="absolute right-3 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground" />
							</div>

							{/* Type Filter */}
							<Select value={backupTypeFilter} onValueChange={(val) => val && setBackupTypeFilter(val)}>
								<SelectTrigger size="sm" className="w-[145px] text-xs font-semibold h-9">
									<SelectValue>
										{backupTypeFilter === 'all'
											? 'Type: All Types'
											: backupTypeFilter === 'volume'
												? 'Type: Volume'
												: 'Type: Database'}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="all">All Types</SelectItem>
									<SelectItem value="volume">Volume</SelectItem>
									<SelectItem value="database">Database</SelectItem>
								</SelectContent>
							</Select>

							{/* Sort Filter */}
							<Select value={backupSort} onValueChange={(val) => val && setBackupSort(val)}>
								<SelectTrigger size="sm" className="w-[155px] text-xs font-semibold h-9">
									<SelectValue>
										{backupSort === 'newest' ? 'Sort: Newest First' : 'Sort: Oldest First'}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="newest">Sort: Newest First</SelectItem>
									<SelectItem value="oldest">Sort: Oldest First</SelectItem>
								</SelectContent>
							</Select>
						</div>
					</div>

					{filteredBackups.length === 0 ? (
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
									{filteredBackups.map((b: any) => (
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

				{/* 3. Domains Tab Table with Dokploy Toolbar */}
				<TabsContent value="domains" className="space-y-4">
					{/* Toolbar Filters */}
					<div className="flex flex-wrap items-center justify-between gap-4">
						<h3 className="text-lg font-bold tracking-tight text-foreground">
							Domains{' '}
							<span className="text-sm font-normal text-muted-foreground">
								({filteredDomains.length})
							</span>
						</h3>

						<div className="flex flex-wrap items-center gap-2.5">
							{/* Search */}
							<div className="relative">
								<Input
									placeholder="Filter domains..."
									value={domainSearch}
									onChange={(e) => setDomainSearch(e.target.value)}
									className="pr-9 w-[190px] h-9 text-xs"
								/>
								<Search className="absolute right-3 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground" />
							</div>

							{/* SSL Filter */}
							<Select value={domainSslFilter} onValueChange={(val) => val && setDomainSslFilter(val)}>
								<SelectTrigger size="sm" className="w-[145px] text-xs font-semibold h-9">
									<SelectValue>
										{domainSslFilter === 'all'
											? 'SSL: All'
											: domainSslFilter === 'https'
												? 'SSL: HTTPS Only'
												: 'SSL: HTTP Only'}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="all">SSL: All</SelectItem>
									<SelectItem value="https">SSL: HTTPS Only</SelectItem>
									<SelectItem value="http">SSL: HTTP Only</SelectItem>
								</SelectContent>
							</Select>

							{/* Sort Filter */}
							<Select value={domainSort} onValueChange={(val) => val && setDomainSort(val)}>
								<SelectTrigger size="sm" className="w-[155px] text-xs font-semibold h-9">
									<SelectValue>
										{domainSort === 'newest' ? 'Sort: Newest First' : 'Sort: Oldest First'}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="newest">Sort: Newest First</SelectItem>
									<SelectItem value="oldest">Sort: Oldest First</SelectItem>
								</SelectContent>
							</Select>
						</div>
					</div>

					{filteredDomains.length === 0 ? (
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
									{filteredDomains.map((dom: any) => {
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

				{/* 4. Deployments Tab (Deployments History vs Queue Sub-tabs with Dokploy Toolbar) */}
				<TabsContent value="deployments" className="space-y-4">
					<div className="flex flex-wrap items-center justify-between gap-4 border-b border-border/60 pb-3">
						<div className="flex items-center gap-2">
							<Button
								variant={deploymentSubTab === 'history' ? 'secondary' : 'ghost'}
								size="sm"
								onClick={() => setDeploymentSubTab('history')}
								className="h-8 text-xs font-bold"
							>
								Deployments ({filteredDeploymentsHistory.length})
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

						{deploymentSubTab === 'history' && (
							<div className="flex flex-wrap items-center gap-2.5">
								{/* Search */}
								<div className="relative">
									<Input
										placeholder="Filter deployments..."
										value={deploySearch}
										onChange={(e) => setDeploySearch(e.target.value)}
										className="pr-9 w-[190px] h-9 text-xs"
									/>
									<Search className="absolute right-3 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground" />
								</div>

								{/* Status Filter */}
								<Select value={deployStatusFilter} onValueChange={(val) => val && setDeployStatusFilter(val)}>
									<SelectTrigger size="sm" className="w-[150px] text-xs font-semibold h-9">
										<SelectValue>
											{deployStatusFilter === 'all'
												? 'Status: All Statuses'
												: deployStatusFilter === 'success'
													? 'Status: Success'
													: deployStatusFilter === 'failed'
														? 'Status: Failed'
														: 'Status: Running'}
										</SelectValue>
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="all">All Statuses</SelectItem>
										<SelectItem value="success">Success</SelectItem>
										<SelectItem value="failed">Failed</SelectItem>
										<SelectItem value="running">Running</SelectItem>
									</SelectContent>
								</Select>
							</div>
						)}
					</div>

					{deploymentSubTab === 'history' ? (
						filteredDeploymentsHistory.length === 0 ? (
							<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
								<Rocket className="size-8 text-muted-foreground/40 mb-2" />
								<p className="text-sm font-semibold text-foreground">No deployment history found</p>
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
										{filteredDeploymentsHistory.map((dep: any) => (
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
