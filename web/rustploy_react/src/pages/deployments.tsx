import * as React from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {
	Rocket,
	Boxes,
	Search,
	RefreshCw,
	FileText,
	Terminal,
	Copy,
	Check,
	XCircle,
	AlertCircle,
	Database,
	Calendar,
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
	const [sortBy, setSortBy] = React.useState<SortKey>('created_at');
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
			refetchInterval: 5000, // Auto refresh every 5s
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
			if (sortBy === 'created_at') {
				return sortDir === 'desc' ? b.created_at - a.created_at : a.created_at - b.created_at;
			}
			if (sortBy === 'title') {
				return sortDir === 'desc' ? b.title.localeCompare(a.title) : a.title.localeCompare(b.title);
			}
			if (sortBy === 'status') {
				return sortDir === 'desc' ? b.status.localeCompare(a.status) : a.status.localeCompare(b.status);
			}
			return 0;
		});
	}, [deployments, searchQuery, statusFilter, typeFilter, sortBy, sortDir]);

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

					{/* Sort Select */}
					<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
						<Select
							value={`${sortBy}-${sortDir}`}
							onValueChange={val => {
								const [key, dir] = val.split('-');
								setSortBy(key as SortKey);
								setSortDir(dir as 'asc' | 'desc');
							}}>
							<SelectTrigger className="w-full sm:w-[160px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
								<SelectValue placeholder="Sort by" />
							</SelectTrigger>
							<SelectContent className="bg-card border-border">
								<SelectItem value="created_at-desc">Newest First</SelectItem>
								<SelectItem value="created_at-asc">Oldest First</SelectItem>
								<SelectItem value="title-asc">Title (A-Z)</SelectItem>
								<SelectItem value="title-desc">Title (Z-A)</SelectItem>
							</SelectContent>
						</Select>
					</div>
				</div>
			</div>

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
					{filteredAndSorted.map(d => {
						const hasApp = d.application_id !== null && d.application_id !== undefined;
						const hasCompose = d.compose_id !== null && d.compose_id !== undefined;
						const hasDatabase = d.database_id !== null && d.database_id !== undefined;
						const type = hasApp ? 'Application' : hasCompose ? 'Compose' : hasDatabase ? 'Database' : 'Generic';
						const isRunningOrQueued = d.status.toUpperCase() === 'RUNNING' || d.status.toUpperCase() === 'QUEUED';
						const status = d.status.toUpperCase();

						// Colors accents for statuses
						const statusColorMap: Record<
							string,
							{border: string; text: string; bg: string; dot: string}
						> = {
							DONE: {
								border: 'border-l-emerald-500/80 hover:border-emerald-500/50',
								text: 'text-emerald-500',
								bg: 'bg-emerald-500/5',
								dot: 'bg-emerald-500',
							},
							SUCCESS: {
								border: 'border-l-emerald-500/80 hover:border-emerald-500/50',
								text: 'text-emerald-500',
								bg: 'bg-emerald-500/5',
								dot: 'bg-emerald-500',
							},
							RUNNING: {
								border: 'border-l-blue-500/80 hover:border-blue-500/50',
								text: 'text-blue-500',
								bg: 'bg-blue-500/5',
								dot: 'bg-blue-500 animate-ping',
							},
							QUEUED: {
								border: 'border-l-zinc-500/80 hover:border-zinc-500/50',
								text: 'text-zinc-400',
								bg: 'bg-zinc-500/5',
								dot: 'bg-zinc-500',
							},
							ERROR: {
								border: 'border-l-rose-500/80 hover:border-rose-500/50',
								text: 'text-rose-500',
								bg: 'bg-rose-500/5',
								dot: 'bg-rose-500',
							},
						};

						const colors = statusColorMap[status] || {
							border: 'border-l-zinc-500/80 hover:border-zinc-500/50',
							text: 'text-zinc-400',
							bg: 'bg-zinc-500/5',
							dot: 'bg-zinc-500',
						};

						return (
							<div
								key={d.id}
								className={cn(
									'group border border-border border-l-4 bg-card/30 hover:bg-card/65 rounded-xl p-4.5 transition-all duration-200 shadow-sm flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4',
									colors.border,
								)}>
								{/* Left Section: Icon and Details */}
								<div className="flex items-start gap-3.5 min-w-0">
									<div className="p-2 rounded-lg bg-muted text-muted-foreground shrink-0 mt-0.5">
										{hasApp ? (
											<Rocket className="size-4.5" />
										) : hasCompose ? (
											<Boxes className="size-4.5" />
										) : (
											<Database className="size-4.5" />
										)}
									</div>
									<div className="min-w-0">
										<div className="flex items-center gap-2 flex-wrap">
											<span className="font-semibold text-sm text-foreground group-hover:text-primary transition-colors">
												{d.title}
											</span>
											<span className="text-[10px] font-mono bg-muted/65 text-muted-foreground px-1.5 py-0.5 rounded border border-border/40">
												#{d.id}
											</span>
											<span className="text-[10px] text-muted-foreground/80 font-medium">
												• {type}
											</span>
										</div>
										<p className="text-xs text-muted-foreground mt-1 line-clamp-2 leading-relaxed">
											{d.description}
										</p>
										{d.error_message && (
											<p className="text-[11px] text-rose-500/90 font-medium mt-1.5 flex items-center gap-1 bg-rose-500/5 border border-rose-500/10 px-2 py-0.5 rounded">
												<AlertCircle className="size-3 shrink-0" />
												{d.error_message}
											</p>
										)}
									</div>
								</div>

								{/* Right Section: Status, Time, and Actions */}
								<div className="flex flex-row sm:flex-col md:flex-row items-center justify-between sm:justify-end gap-4 w-full sm:w-auto shrink-0 border-t border-border/10 sm:border-t-0 pt-3 sm:pt-0">
									<div className="flex flex-col items-start sm:items-end gap-1.5">
										{/* Status Badge */}
										<div className="flex items-center gap-1.5">
											<span className={cn('size-1.5 rounded-full shrink-0', colors.dot)} />
											{status === 'RUNNING' && (
												<span className="size-1.5 bg-blue-500 rounded-full shrink-0 absolute opacity-75 animate-ping" />
											)}
											<span className={cn('text-[10px] font-bold tracking-wider uppercase', colors.text)}>
												{d.status}
											</span>
										</div>
										{/* Date & Time */}
										<span className="text-[10px] text-muted-foreground/85 flex items-center gap-1">
											<Calendar className="size-3" />
											{new Date(d.created_at * 1000).toLocaleString()}
										</span>
									</div>

									{/* Action buttons */}
									<div className="flex items-center gap-1.5">
										<Button
											variant="ghost"
											size="sm"
											onClick={() => setSelectedDeployment(d)}
											className="h-8 px-2.5 rounded-lg hover:bg-muted text-xs font-semibold flex items-center gap-1.5 text-muted-foreground hover:text-foreground border border-transparent hover:border-border/30">
											<Terminal className="size-3.5" />
											Logs
										</Button>

										{isRunningOrQueued && (
											<Button
												variant="ghost"
												size="icon"
												onClick={() => handleCancelDeployment(d.id)}
												className="size-8 rounded-lg hover:bg-rose-500/5 hover:text-rose-500 text-muted-foreground/60 border border-transparent hover:border-rose-500/10">
												<XCircle className="size-4" />
											</Button>
										)}
									</div>
								</div>
							</div>
						);
					})}
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
