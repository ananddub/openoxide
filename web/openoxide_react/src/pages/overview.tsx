import {createFileRoute} from '@tanstack/react-router';
import {useState, useMemo} from 'react';
import {
	Box,
	Globe,
	Clock,
	Rocket,
	Loader2,
	ExternalLink,
	Search,
	Trash2,
} from 'lucide-react';
import {
	Tabs,
	TabsList,
	TabsTrigger,
	TabsContent,
} from '#/components/ui/tabs';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
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
import {useAppStore} from '#/stores/app-store';
import {useDeployments} from '#/hooks/deployments/use-deployments';
import {DeploymentItem} from '#/components/deployments/deployment-item';
import {DeploymentLogsDialog} from '#/components/deployments/deployment-logs-dialog';
import {OverviewServicesTab} from '#/components/overview/overview-services-tab';
import {useDeploymentRunning} from 'virtual:openoxide-live';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState('services');
	const [deploymentSubTab, setDeploymentSubTab] = useState<
		'history' | 'queue'
	>('history');

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

	const rawDomains = useAppStore(state => state.domains);
	const rawBackups = useAppStore(state => state.backups);

	// Live running deployments queue
	const {data: runningDeployments = []} = useDeploymentRunning();

	const {
		filteredAndSorted: rawDeploymentsList,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		copied,
		handleCopyLogs,
		handleCancelDeployment,
		handleDeleteDeployment,
		handleClearAllDeployments,
	} = useDeployments();

	// Filter & Sort Backups
	const filteredBackups = useMemo(() => {
		let list = (rawBackups || []).filter((b: any) => {
			const matchesSearch =
				!backupSearch ||
				(b.name || '')
					.toLowerCase()
					.includes(backupSearch.toLowerCase()) ||
				(b.destination || '')
					.toLowerCase()
					.includes(backupSearch.toLowerCase());

			const matchesType =
				backupTypeFilter === 'all' ||
				(backupTypeFilter === 'volume' &&
					(b.backup_type || '').toLowerCase().includes('volume')) ||
				(backupTypeFilter === 'database' &&
					(b.backup_type || '').toLowerCase().includes('database'));

			return matchesSearch && matchesType;
		});

		if (backupSort === 'newest')
			list.sort(
				(a, b) => Number(b.created_at || 0) - Number(a.created_at || 0),
			);
		else if (backupSort === 'oldest')
			list.sort(
				(a, b) => Number(a.created_at || 0) - Number(b.created_at || 0),
			);

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

		if (domainSort === 'newest')
			list.sort((a, b) => Number(b.id || 0) - Number(a.id || 0));
		else if (domainSort === 'oldest')
			list.sort((a, b) => Number(a.id || 0) - Number(b.id || 0));

		return list;
	}, [rawDomains, domainSearch, domainSslFilter, domainSort]);

	// Filter Deployments History
	const filteredDeploymentsHistory = useMemo(() => {
		return (rawDeploymentsList || []).filter((d: any) => {
			const titleStr = (d.title || d.description || '').toLowerCase();
			const matchesSearch =
				!deploySearch || titleStr.includes(deploySearch.toLowerCase());

			const statusStr = (d.status || '').toLowerCase();
			const matchesStatus =
				deployStatusFilter === 'all' ||
				(deployStatusFilter === 'success' &&
					(statusStr === 'done' ||
						statusStr === 'healthy' ||
						statusStr === 'success')) ||
				(deployStatusFilter === 'failed' &&
					(statusStr === 'error' || statusStr === 'failed')) ||
				(deployStatusFilter === 'running' &&
					(statusStr === 'running' || statusStr === 'queued'));

			return matchesSearch && matchesStatus;
		});
	}, [rawDeploymentsList, deploySearch, deployStatusFilter]);

	return (
		<div className="flex w-full max-w-full animate-in flex-col gap-6 p-6 duration-200 fade-in md:px-8">
			{/* Page Header */}
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold tracking-tight text-foreground">
					Overview
				</h1>
				<p className="text-xs text-muted-foreground">
					Centralized platform overview of all applications, compose
					stacks, databases, volume backups, domains, and deployments
				</p>
			</div>

			{/* Main Overview Tabs (Services -> Backups -> Domains -> Deployments) */}
			<Tabs
				value={activeTab}
				onValueChange={setActiveTab}
				className="w-full space-y-6">
				<TabsList
					variant="line"
					className="w-full justify-start gap-6 rounded-none border-b border-border pb-0">
					<TabsTrigger
						value="services"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Box className="size-4" />
						Services
					</TabsTrigger>
					<TabsTrigger
						value="backups"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Clock className="size-4" />
						Backups
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
							{filteredBackups.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger
						value="domains"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Globe className="size-4" />
						Domains
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
							{filteredDomains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger
						value="deployments"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Rocket className="size-4" />
						Deployments
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
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
									onChange={e => setBackupSearch(e.target.value)}
									className="h-9 w-[190px] pr-9 text-xs"
								/>
								<Search className="absolute top-1/2 right-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
							</div>

							{/* Type Filter */}
							<Select
								value={backupTypeFilter}
								onValueChange={val => val && setBackupTypeFilter(val)}>
								<SelectTrigger
									size="sm"
									className="h-9 w-[145px] text-xs font-semibold">
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
							<Select
								value={backupSort}
								onValueChange={val => val && setBackupSort(val)}>
								<SelectTrigger
									size="sm"
									className="h-9 w-[155px] text-xs font-semibold">
									<SelectValue>
										{backupSort === 'newest'
											? 'Sort: Newest First'
											: 'Sort: Oldest First'}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="newest">
										Sort: Newest First
									</SelectItem>
									<SelectItem value="oldest">
										Sort: Oldest First
									</SelectItem>
								</SelectContent>
							</Select>
						</div>
					</div>

					{filteredBackups.length === 0 ? (
						<div className="flex flex-col items-center justify-center rounded-xl border border-dashed p-12 text-center">
							<Clock className="mb-2 size-8 text-muted-foreground/40" />
							<p className="text-sm font-semibold text-foreground">
								No volume backups found
							</p>
							<p className="text-xs text-muted-foreground">
								Scheduled volume and database backups will appear here
							</p>
						</div>
					) : (
						<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border border-border/60 bg-card shadow-xs">
							<Table>
								<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
									<TableRow className="border-b border-border/60 hover:bg-transparent">
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Backup Name
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Type
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Status
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Destination
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Created
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Actions
										</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{filteredBackups.map((b: any) => (
										<TableRow
											key={b.id}
											className="border-b border-border/40 transition-colors hover:bg-muted/40">
											<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
												<div className="flex items-center gap-2">
													<Clock className="size-4 shrink-0 text-primary" />
													<span>{b.name || `Backup #${b.id}`}</span>
												</div>
											</TableCell>
											<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
												<Badge
													variant="outline"
													className="font-mono text-[10px]">
													{b.backup_type || 'Volume'}
												</Badge>
											</TableCell>
											<TableCell className="px-4 py-3.5">
												<div className="inline-flex items-center gap-1.5 rounded-full bg-emerald-500/10 px-2.5 py-1 text-xs font-semibold text-emerald-500">
													<span className="size-2 shrink-0 rounded-full bg-emerald-500" />
													<span>{b.status || 'DONE'}</span>
												</div>
											</TableCell>
											<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
												{b.destination || 'Local Storage'}
											</TableCell>
											<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
												{b.created_at
													? new Date(
															Number(b.created_at) * 1000,
														).toLocaleDateString()
													: 'N/A'}
											</TableCell>
											<TableCell className="px-4 py-3.5 text-right">
												<Button
													size="sm"
													variant="outline"
													className="h-7 px-2.5 text-xs font-semibold">
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
									onChange={e => setDomainSearch(e.target.value)}
									className="h-9 w-[190px] pr-9 text-xs"
								/>
								<Search className="absolute top-1/2 right-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
							</div>

							{/* SSL Filter */}
							<Select
								value={domainSslFilter}
								onValueChange={val => val && setDomainSslFilter(val)}>
								<SelectTrigger
									size="sm"
									className="h-9 w-[145px] text-xs font-semibold">
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
							<Select
								value={domainSort}
								onValueChange={val => val && setDomainSort(val)}>
								<SelectTrigger
									size="sm"
									className="h-9 w-[155px] text-xs font-semibold">
									<SelectValue>
										{domainSort === 'newest'
											? 'Sort: Newest First'
											: 'Sort: Oldest First'}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="newest">
										Sort: Newest First
									</SelectItem>
									<SelectItem value="oldest">
										Sort: Oldest First
									</SelectItem>
								</SelectContent>
							</Select>
						</div>
					</div>

					{filteredDomains.length === 0 ? (
						<div className="flex flex-col items-center justify-center rounded-xl border border-dashed p-12 text-center">
							<Globe className="mb-2 size-8 text-muted-foreground/40" />
							<p className="text-sm font-semibold text-foreground">
								No active domains configured
							</p>
							<p className="text-xs text-muted-foreground">
								Configure custom domain routes in application settings
							</p>
						</div>
					) : (
						<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border border-border/60 bg-card shadow-xs">
							<Table>
								<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
									<TableRow className="border-b border-border/60 hover:bg-transparent">
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Domain / Host
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Target Service
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Path & Port
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											SSL Status
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Project
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Actions
										</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{filteredDomains.map((dom: any) => {
										const fullUrl = `${dom.https ? 'https' : 'http'}://${dom.domain || dom.host}${dom.path || ''}`;
										return (
											<TableRow
												key={dom.id}
												className="border-b border-border/40 transition-colors hover:bg-muted/40">
												<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
													<div className="flex items-center gap-2">
														<Globe className="size-4 shrink-0 text-primary" />
														<a
															href={fullUrl}
															target="_blank"
															rel="noreferrer"
															className="transition-colors hover:text-primary hover:underline">
															{dom.domain || dom.host}
														</a>
													</div>
												</TableCell>
												<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
													{dom.service_name || 'Application'}
												</TableCell>
												<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
													{dom.path || '/'} · Port: {dom.port || 80}
												</TableCell>
												<TableCell className="px-4 py-3.5">
													{dom.https ? (
														<Badge
															variant="outline"
															className="border-emerald-500/30 bg-emerald-500/10 text-[10px] font-semibold text-emerald-500">
															HTTPS (SSL)
														</Badge>
													) : (
														<Badge
															variant="secondary"
															className="text-[10px] font-semibold">
															HTTP
														</Badge>
													)}
												</TableCell>
												<TableCell className="px-4 py-3.5 text-xs font-medium text-muted-foreground">
									{dom.project_name || 'OpenOxide Project'}
												</TableCell>
												<TableCell className="px-4 py-3.5 text-right">
													<a
														href={fullUrl}
														target="_blank"
														rel="noreferrer"
														className="inline-flex items-center gap-1 text-xs font-semibold text-primary hover:underline">
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
								variant={
									deploymentSubTab === 'history' ? 'secondary' : 'ghost'
								}
								size="sm"
								onClick={() => setDeploymentSubTab('history')}
								className="h-8 text-xs font-bold">
								Deployments ({filteredDeploymentsHistory.length})
							</Button>
							<Button
								variant={
									deploymentSubTab === 'queue' ? 'secondary' : 'ghost'
								}
								size="sm"
								onClick={() => setDeploymentSubTab('queue')}
								className="flex h-8 items-center gap-1.5 text-xs font-bold">
								{runningDeployments.length > 0 && (
									<Loader2 className="size-3 animate-spin text-amber-500" />
								)}
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
										onChange={e => setDeploySearch(e.target.value)}
										className="h-9 w-[190px] pr-9 text-xs"
									/>
									<Search className="absolute top-1/2 right-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
								</div>

								{/* Status Filter */}
								<Select
									value={deployStatusFilter}
									onValueChange={val => val && setDeployStatusFilter(val)}>
									<SelectTrigger
										size="sm"
										className="h-9 w-[150px] text-xs font-semibold">
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

								<Button
									variant="outline"
									size="sm"
									onClick={handleClearAllDeployments}
									className="flex h-9 items-center gap-1.5 border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive hover:bg-destructive/20">
									<Trash2 className="size-3.5" /> Clear History
								</Button>
							</div>
						)}
					</div>

					{deploymentSubTab === 'history' ? (
						filteredDeploymentsHistory.length === 0 ? (
							<div className="flex flex-col items-center justify-center rounded-xl border border-dashed p-12 text-center">
								<Rocket className="mb-2 size-8 text-muted-foreground/40" />
								<p className="text-sm font-semibold text-foreground">
									No deployment history found
								</p>
								<p className="text-xs text-muted-foreground">
									Deployments will appear here once triggered
								</p>
							</div>
						) : (
							<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border border-border/60 bg-card shadow-xs">
								<Table>
									<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
										<TableRow className="border-b border-border/60 hover:bg-transparent">
											<TableHead className="sticky top-0 z-20 w-[80px] bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
												ID
											</TableHead>
											<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
												Deployment
											</TableHead>
											<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
												Type
											</TableHead>
											<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
												Status
											</TableHead>
											<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
												Created
											</TableHead>
											<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
												Actions
											</TableHead>
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
												onDelete={handleDeleteDeployment}
											/>
										))}
									</TableBody>
								</Table>
							</div>
						)
					) : runningDeployments.length === 0 ? (
						<div className="flex flex-col items-center justify-center rounded-xl border border-dashed p-12 text-center">
							<Rocket className="mb-2 size-8 text-muted-foreground/40" />
							<p className="text-sm font-semibold text-foreground">
								Queue is empty
							</p>
							<p className="text-xs text-muted-foreground">
								Active or building deployments will appear here in real
								time
							</p>
						</div>
					) : (
						<div className="max-h-[calc(100vh-340px)] min-h-[280px] overflow-y-auto rounded-xl border border-border/60 bg-card shadow-xs">
							<Table>
								<TableHeader className="sticky top-0 z-20 bg-card/95 backdrop-blur-md">
									<TableRow className="border-b border-border/60 hover:bg-transparent">
										<TableHead className="sticky top-0 z-20 w-[80px] bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											ID
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Deployment
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Type
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Status
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Created
										</TableHead>
										<TableHead className="sticky top-0 z-20 bg-card/95 px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase backdrop-blur-md">
											Actions
										</TableHead>
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
