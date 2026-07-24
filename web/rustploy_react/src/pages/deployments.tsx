import * as React from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {
	Rocket,
	Boxes,
	Search,
	ArrowUpDown,
	RefreshCw,
	FileText,
	Terminal,
	Copy,
	Check,
	XCircle,
	AlertCircle,
	Database,
} from 'lucide-react';
import {toast} from 'sonner';

import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {formatApiError, cn} from '#/api/utils';

export const Route = createFileRoute('/_app/Deployments')({
	component: DeploymentsPage,
});

type SortKey = 'created_at' | 'title' | 'status';

function DeploymentsPage() {
	const queryClient = useQueryClient();
	const [refreshing, setRefreshing] = React.useState(false);

	// Filters and sorting state
	const [searchQuery, setSearchQuery] = React.useState('');
	const [statusFilter, setStatusFilter] = React.useState('all');
	const [typeFilter, setTypeFilter] = React.useState('all');
	const [sortKey, setSortKey] = React.useState<SortKey>('created_at');
	const [sortDir, setSortDir] = React.useState<'asc' | 'desc'>('desc');

	// Logs Dialog state
	const [selectedDeployment, setSelectedDeployment] = React.useState<any>(null);
	const [logs, setLogs] = React.useState<string>('');
	const [isLogsLoading, setIsLogsLoading] = React.useState(false);
	const [copied, setCopied] = React.useState(false);

	const cancelMutation = $api.useMutation('post', '/deployments/{id}/cancel');

	// Fetch deployments from backend
	const {
		data: deployments,
		isLoading,
		refetch,
	} = $api.useQuery(
		'get',
		'/deployments',
		{
			query: {
				query: {
					limit: 100,
				},
			},
		},
		{
			refetchInterval: 5000, // Auto refresh every 5s to update progress
		},
	);

	const handleRefresh = async () => {
		setRefreshing(true);
		try {
			await refetch();
			toast.success('Deployments list updated');
		} catch {
			toast.error('Failed to update deployments');
		} finally {
			setRefreshing(false);
		}
	};

	const handleCancelDeployment = async (id: number) => {
		if (!confirm('Are you sure you want to cancel this deployment?')) return;
		try {
			await cancelMutation.mutateAsync({
				params: {
					path: {
						id,
					},
				},
			});
			toast.success('Deployment cancellation requested');
			refetch();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	// Copy logs helper
	const handleCopyLogs = () => {
		navigator.clipboard.writeText(logs);
		setCopied(true);
		toast.success('Logs copied to clipboard');
		setTimeout(() => setCopied(false), 2000);
	};

	// Parse logs reader
	React.useEffect(() => {
		if (!selectedDeployment) return;

		let isMounted = true;
		let controller = new AbortController();
		setLogs('');
		setIsLogsLoading(true);

		const readLogs = async () => {
			try {
				const token = localStorage.getItem('rustploy-auth-session');
				const response = await fetch(
					`http://das.tail25b5a0.ts.net:4000/deployments/${selectedDeployment.id}/logs`,
					{
						headers: {
							Authorization: token ? `Bearer ${token}` : '',
						},
						signal: controller.signal,
					},
				);

				if (!response.ok) {
					throw new Error('Failed to fetch logs');
				}

				setIsLogsLoading(false);
				const reader = response.body?.getReader();
				const decoder = new TextDecoder();

				if (!reader) return;

				let buffer = '';
				while (isMounted) {
					const {done, value} = await reader.read();
					if (done) break;

					buffer += decoder.decode(value, {stream: true});
					const lines = buffer.split('\n');
					buffer = lines.pop() || '';

					for (const line of lines) {
						if (line.startsWith('data:')) {
							try {
								const jsonStr = line.slice(5).trim();
								if (jsonStr) {
									const data = JSON.parse(jsonStr);
									if (data.line) {
										setLogs(prev => prev + data.line + '\n');
									} else if (data.message) {
										setLogs(prev => prev + data.message + '\n');
									}
								}
							} catch {
								setLogs(prev => prev + line.slice(5) + '\n');
							}
						} else if (line.trim()) {
							setLogs(prev => prev + line + '\n');
						}
					}
				}
			} catch (err: any) {
				if (err.name !== 'AbortError') {
					toast.error('Failed to load logs');
					setIsLogsLoading(false);
				}
			}
		};

		readLogs();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [selectedDeployment]);

	// Filter & Sort list
	const filteredAndSorted = React.useMemo(() => {
		if (!deployments) return [];

		// 1. Filtering
		let result = deployments.filter(d => {
			const q = searchQuery.toLowerCase();
			const matchesSearch =
				d.title.toLowerCase().includes(q) ||
				(d.description || '').toLowerCase().includes(q) ||
				(d.error_message || '').toLowerCase().includes(q);

			const status = d.status.toUpperCase();
			const matchesStatus =
				statusFilter === 'all' ||
				(statusFilter === 'running' && status === 'RUNNING') ||
				(statusFilter === 'queued' && status === 'QUEUED') ||
				(statusFilter === 'done' && status === 'DONE') ||
				(statusFilter === 'error' && status === 'ERROR');

			const hasApp = d.application_id !== null && d.application_id !== undefined;
			const hasCompose = d.compose_id !== null && d.compose_id !== undefined;
			const hasDatabase = d.database_id !== null && d.database_id !== undefined;

			const matchesType =
				typeFilter === 'all' ||
				(typeFilter === 'application' && hasApp) ||
				(typeFilter === 'compose' && hasCompose) ||
				(typeFilter === 'database' && hasDatabase);

			return matchesSearch && matchesStatus && matchesType;
		});

		// 2. Sorting
		return [...result].sort((a, b) => {
			let av: any = a[sortKey] || '';
			let bv: any = b[sortKey] || '';

			if (sortKey === 'created_at') {
				return sortDir === 'asc' ? a.created_at - b.created_at : b.created_at - a.created_at;
			}

			if (typeof av === 'string' && typeof bv === 'string') {
				return sortDir === 'asc' ? av.localeCompare(bv) : bv.localeCompare(av);
			}

			return 0;
		});
	}, [deployments, searchQuery, statusFilter, typeFilter, sortKey, sortDir]);

	const toggleSort = (key: SortKey) => {
		if (sortKey === key) {
			setSortDir(prev => (prev === 'asc' ? 'desc' : 'asc'));
		} else {
			setSortKey(key);
			setSortDir('asc');
		}
	};

	const getStatusBadgeClass = (status: string) => {
		const stat = status.toUpperCase();
		if (stat === 'DONE' || stat === 'SUCCESS') {
			return 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20';
		}
		if (stat === 'RUNNING') {
			return 'bg-blue-500/10 text-blue-500 border-blue-500/20 animate-pulse';
		}
		if (stat === 'QUEUED') {
			return 'bg-zinc-500/10 text-zinc-500 border-zinc-500/20';
		}
		return 'bg-rose-500/10 text-rose-500 border-rose-500/20';
	};

	return (
		<div className="flex flex-col gap-6 w-full">
			{/* Page Header */}
			<div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/40 pb-6">
				<div>
					<h1 className="text-3xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/75 bg-clip-text">
						Deployments
					</h1>
					<p className="text-muted-foreground mt-1.5 text-sm">
						Monitor deployment history, execution states, and live streaming console logs.
					</p>
				</div>

				<div className="flex items-center gap-3">
					<Button
						variant="outline"
						onClick={handleRefresh}
						disabled={refreshing}
						className="border-border bg-card/40 hover:bg-card/70 font-semibold h-10 px-4 rounded-lg flex items-center gap-2">
						<RefreshCw className={cn('size-4', refreshing && 'animate-spin')} />
						Refresh
					</Button>
				</div>
			</div>

			{/* Filters Control Bar */}
			<div className="flex flex-col gap-4 animate-in fade-in duration-200">
				<div className="flex flex-col sm:flex-row items-center gap-3">
					{/* Search */}
					<div className="relative w-full sm:grow">
						<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
						<Input
							placeholder="Search by title, description or logs..."
							value={searchQuery}
							onChange={e => setSearchQuery(e.target.value)}
							className="pl-9 bg-card/45 border-border/80 h-10 w-full"
						/>
					</div>

					{/* Status Select */}
					<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
						<Select value={statusFilter} onValueChange={setStatusFilter}>
							<SelectTrigger className="w-full sm:w-[150px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
								<SelectValue placeholder="Status" />
							</SelectTrigger>
							<SelectContent className="bg-card border-border">
								<SelectItem value="all">All statuses</SelectItem>
								<SelectItem value="running">Running</SelectItem>
								<SelectItem value="queued">Queued</SelectItem>
								<SelectItem value="done">Completed</SelectItem>
								<SelectItem value="error">Failed</SelectItem>
							</SelectContent>
						</Select>
					</div>

					{/* Type Select */}
					<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
						<Select value={typeFilter} onValueChange={setTypeFilter}>
							<SelectTrigger className="w-full sm:w-[150px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
								<SelectValue placeholder="Type" />
							</SelectTrigger>
							<SelectContent className="bg-card border-border">
								<SelectItem value="all">All types</SelectItem>
								<SelectItem value="application">Application</SelectItem>
								<SelectItem value="compose">Compose</SelectItem>
								<SelectItem value="database">Database</SelectItem>
							</SelectContent>
						</Select>
					</div>
				</div>
			</div>

			{/* Table View */}
			{isLoading ? (
				<div className="flex flex-col gap-3 py-10 items-center justify-center">
					<RefreshCw className="size-8 animate-spin text-primary" />
					<p className="text-sm text-muted-foreground font-medium">Loading deployments...</p>
				</div>
			) : filteredAndSorted.length > 0 ? (
				<div className="border border-border bg-card/25 rounded-2xl overflow-hidden shadow-sm backdrop-blur-sm animate-in fade-in duration-200">
					<Table>
						<TableHeader className="bg-muted/40">
							<TableRow className="border-b border-border/50 hover:bg-transparent">
								<TableHead className="w-16">ID</TableHead>
								<TableHead>
									<button
										onClick={() => toggleSort('title')}
										className="flex items-center gap-1 hover:text-foreground font-semibold text-xs tracking-wider uppercase">
										Deployment Details
										<ArrowUpDown className="size-3 opacity-60" />
									</button>
								</TableHead>
								<TableHead className="w-32">Type</TableHead>
								<TableHead className="w-36">
									<button
										onClick={() => toggleSort('status')}
										className="flex items-center gap-1 hover:text-foreground font-semibold text-xs tracking-wider uppercase">
										Status
										<ArrowUpDown className="size-3 opacity-60" />
									</button>
								</TableHead>
								<TableHead className="w-48">
									<button
										onClick={() => toggleSort('created_at')}
										className="flex items-center gap-1 hover:text-foreground font-semibold text-xs tracking-wider uppercase">
										Started
										<ArrowUpDown className="size-3 opacity-60" />
									</button>
								</TableHead>
								<TableHead className="w-32 text-right">Actions</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{filteredAndSorted.map(d => {
								const hasApp = d.application_id !== null && d.application_id !== undefined;
								const hasCompose = d.compose_id !== null && d.compose_id !== undefined;
								const hasDatabase = d.database_id !== null && d.database_id !== undefined;
								const type = hasApp ? 'Application' : hasCompose ? 'Compose' : hasDatabase ? 'Database' : 'Generic';
								const isRunningOrQueued = d.status.toUpperCase() === 'RUNNING' || d.status.toUpperCase() === 'QUEUED';

								return (
									<TableRow key={d.id} className="border-b border-border/30 hover:bg-card/45 transition-colors duration-150">
										<TableCell className="font-mono text-xs text-muted-foreground">
											#{d.id}
										</TableCell>
										<TableCell>
											<div className="flex flex-col gap-0.5">
												<span className="font-semibold text-sm text-foreground">
													{d.title}
												</span>
												<span className="text-xs text-muted-foreground truncate max-w-lg">
													{d.description}
												</span>
												{d.error_message && (
													<span className="text-xs text-rose-500/80 font-medium truncate max-w-lg flex items-center gap-1">
														<AlertCircle className="size-3" />
														{d.error_message}
													</span>
												)}
											</div>
										</TableCell>
										<TableCell>
											<div className="flex items-center gap-1.5 text-xs text-muted-foreground font-semibold">
												{hasApp ? (
													<Rocket className="size-3.5" />
												) : hasCompose ? (
													<Boxes className="size-3.5" />
												) : (
													<Database className="size-3.5" />
												)}
												{type}
											</div>
										</TableCell>
										<TableCell>
											<span
												className={cn(
													'inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-[10px] font-bold tracking-wide uppercase border',
													getStatusBadgeClass(d.status),
												)}>
												{d.status}
											</span>
										</TableCell>
										<TableCell className="text-xs text-muted-foreground">
											{new Date(d.created_at * 1000).toLocaleString()}
										</TableCell>
										<TableCell className="text-right">
											<div className="flex items-center justify-end gap-2">
												<Button
													variant="ghost"
													size="sm"
													onClick={() => setSelectedDeployment(d)}
													className="h-8 px-2.5 rounded-lg hover:bg-muted text-xs font-semibold flex items-center gap-1 text-muted-foreground hover:text-foreground">
													<Terminal className="size-3.5" />
													Logs
												</Button>

												{isRunningOrQueued && (
													<Button
														variant="ghost"
														size="sm"
														onClick={() => handleCancelDeployment(d.id)}
														className="h-8 px-2 rounded-lg hover:bg-rose-500/5 hover:text-rose-500 text-xs font-bold text-muted-foreground/60">
														<XCircle className="size-3.5" />
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
							onClick={() => {
								setSearchQuery('');
								setStatusFilter('all');
								setTypeFilter('all');
							}}
							className="mt-4 text-xs font-semibold text-primary">
							Clear All Filters
						</Button>
					)}
				</div>
			)}

			{/* Logs Stream Dialog */}
			<Dialog open={!!selectedDeployment} onOpenChange={open => !open && setSelectedDeployment(null)}>
				<DialogContent className="sm:max-w-4xl bg-card border-border h-[650px] flex flex-col justify-between">
					<DialogHeader className="border-b border-border/30 pb-4">
						<div className="flex items-center justify-between pr-6">
							<div>
								<DialogTitle className="text-lg font-bold flex items-center gap-2">
									<Terminal className="size-5 text-primary" />
									Deployment Logs #{selectedDeployment?.id}
								</DialogTitle>
								<DialogDescription className="text-xs text-muted-foreground mt-1">
									{selectedDeployment?.title} - {selectedDeployment?.description}
								</DialogDescription>
							</div>

							<div className="flex items-center gap-2">
								<Button
									variant="outline"
									size="sm"
									onClick={handleCopyLogs}
									className="border-border hover:bg-muted h-8 px-2.5 text-xs font-semibold flex items-center gap-1.5 text-muted-foreground hover:text-foreground">
									{copied ? (
										<>
											<Check className="size-3.5 text-emerald-500" />
											Copied
										</>
									) : (
										<>
											<Copy className="size-3.5" />
											Copy Logs
										</>
									)}
								</Button>
							</div>
						</div>
					</DialogHeader>

					<div className="grow overflow-hidden my-4 relative">
						{isLogsLoading ? (
							<div className="absolute inset-0 flex flex-col items-center justify-center gap-2 bg-[#0c0c0e]/95 rounded-lg border border-border/40 z-10">
								<RefreshCw className="size-6 animate-spin text-primary" />
								<p className="text-xs font-medium text-muted-foreground">
									Connecting to console stream...
								</p>
							</div>
						) : null}

						<pre className="h-full w-full bg-[#0c0c0e] text-zinc-100 border border-zinc-800 rounded-lg p-4 font-mono text-[11px] leading-relaxed overflow-y-auto whitespace-pre-wrap select-text">
							{logs || 'Waiting for build console outputs...'}
						</pre>
					</div>

					<div className="flex justify-end border-t border-border/30 pt-4">
						<Button
							onClick={() => setSelectedDeployment(null)}
							className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold">
							Close Terminal
						</Button>
					</div>
				</DialogContent>
			</Dialog>
		</div>
	);
}
