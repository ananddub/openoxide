import {useState, useEffect} from 'react';
import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {globalServerConnStore, ConnectionStatus} from '#/hooks/use-server-connection-store';
import {
	Server,
	Plug,
	RefreshCw,
	Check,
	X,
	MoreVertical,
} from 'lucide-react';

interface RemoteServersListProps {
	servers: any[];
	sshKeys: any[];
	isLoading: boolean;
	onEditServer: (server: any) => void;
	onDeleteServer: (server: any) => void;
	onSetupServer: (server: any) => void;
	onToggleStatus: (server: any) => void;
	onOpenTerminal?: (server: any) => void;
}

export function RemoteServersList({
	servers,
	isLoading,
	onEditServer,
	onDeleteServer,
	onSetupServer,
	onToggleStatus,
	onOpenTerminal,
}: RemoteServersListProps) {
	const [, forceUpdate] = useState({});
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	useEffect(() => {
		const unsubscribe = globalServerConnStore.subscribe(() => {
			forceUpdate({});
		});
		return unsubscribe;
	}, []);

	const handleTestConnection = async (server: any) => {
		globalServerConnStore.setStatus(server.id, 'testing');
		try {
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''} as any,
			});
			globalServerConnStore.setStatus(server.id, 'success');
			toast.success(`SSH Connection verified for ${server.name}`);
		} catch (err: any) {
			globalServerConnStore.setStatus(server.id, 'failed');
			toast.error(formatApiError(err));
		}
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-4">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-16 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (servers.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-12 text-center flex flex-col items-center justify-center rounded-xl my-4">
				<div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center text-primary mb-3">
					<Server className="w-6 h-6" />
				</div>
				<h3 className="text-sm font-bold text-foreground">No Remote Servers Found</h3>
				<p className="text-xs text-muted-foreground max-w-sm mt-1 mb-4">
					Connect external Linux nodes via SSH for remote application deployment.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-3 w-full">
			{servers.map(item => {
				const status: ConnectionStatus = globalServerConnStore.getStatus(item.id);
				const isTesting = status === 'testing';

				// Global Reactive Status Dot Color
				const getDotColor = () => {
					if (isTesting) return 'bg-amber-400 animate-pulse shadow-sm shadow-amber-400/50';
					if (status === 'success') return 'bg-emerald-500 shadow-sm shadow-emerald-500/50';
					if (status === 'failed') return 'bg-rose-500 shadow-sm shadow-rose-500/50';
					return 'bg-zinc-400/80'; // Default gray dot until tested
				};

				const getStatusTooltip = () => {
					if (isTesting) return 'Testing SSH connection...';
					if (status === 'success') return 'SSH Connection Verified (Green)';
					if (status === 'failed') return 'SSH Connection Failed (Red)';
					return 'SSH Connection Untested (Click Plug icon to verify)';
				};

				return (
					<Card
						key={item.id}
						className="bg-card border-border hover:border-border/80 transition-all rounded-xl shadow-sm"
					>
						<CardContent className="p-3 flex items-center justify-between gap-2">
							{/* Left: Global Reactive Status Dot, Server Name & Host IP */}
							<div className="flex items-center gap-2 min-w-0 flex-1">
								<button
									type="button"
									onClick={() => onToggleStatus(item)}
									title={getStatusTooltip()}
									className="shrink-0"
								>
									<span className={`block w-2.5 h-2.5 rounded-full transition-all ${getDotColor()}`} />
								</button>

								<div className="flex flex-col min-w-0 flex-1">
									<h3 className="text-xs font-bold text-foreground truncate">{item.name}</h3>
									<span className="text-[11px] font-mono text-muted-foreground truncate mt-0.5">
										{item.username || 'root'}@{item.ip_address}:{item.port || 22}
									</span>
								</div>
							</div>

							{/* Right: Plug Test Icon Button & Clean 3-Dots Dropdown */}
							<div className="flex items-center gap-1 shrink-0">
								<Button
									variant="ghost"
									size="icon"
									onClick={() => handleTestConnection(item)}
									disabled={isTesting}
									title={
										isTesting
											? 'Testing SSH Connection...'
											: status === 'success'
												? 'SSH Connection Verified'
												: status === 'failed'
													? 'SSH Connection Failed'
													: 'Test SSH Connection'
									}
									className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0"
								>
									{isTesting ? (
										<RefreshCw className="w-3.5 h-3.5 animate-spin text-primary" />
									) : status === 'success' ? (
										<Check className="w-3.5 h-3.5 text-emerald-500 stroke-[2.5]" />
									) : status === 'failed' ? (
										<X className="w-3.5 h-3.5 text-rose-500 stroke-[2.5]" />
									) : (
										<Plug className="w-3.5 h-3.5" />
									)}
								</Button>

								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-40 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onSetupServer(item)}
										>
											Audit Server
										</DropdownMenuItem>

										{onOpenTerminal && (
											<DropdownMenuItem
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
												onClick={() => onOpenTerminal(item)}
											>
												Open Terminal
											</DropdownMenuItem>
										)}

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onEditServer(item)}
										>
											Edit Server
										</DropdownMenuItem>

										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted/80 text-rose-500 text-xs font-medium"
											onClick={() => onDeleteServer(item)}
										>
											Delete Server
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
